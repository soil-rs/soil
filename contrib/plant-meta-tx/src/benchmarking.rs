// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use subsoil::impl_tx_ext_default;
use topsoil_benchmarking::v2::*;
use topsoil_support::traits::UnfilteredDispatchable;

pub mod types {
	use super::*;
	use subsoil::runtime::traits::DispatchInfoOf;
	use topsoil_support::traits::OriginTrait;

	type CallOf<T> = <T as topsoil_system::Config>::RuntimeCall;

	/// A weightless extension to facilitate the bare dispatch benchmark.
	#[derive(TypeInfo, Eq, PartialEq, Clone, Encode, Decode, DecodeWithMemTracking)]
	#[scale_info(skip_type_params(T))]
	pub struct WeightlessExtension<T>(core::marker::PhantomData<T>);
	impl<T: Config + Send + Sync> core::fmt::Debug for WeightlessExtension<T> {
		fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
			write!(f, "WeightlessExtension")
		}
	}
	impl<T: Config + Send + Sync> Default for WeightlessExtension<T> {
		fn default() -> Self {
			WeightlessExtension(Default::default())
		}
	}
	impl<T: Config + Send + Sync> TransactionExtension<CallOf<T>> for WeightlessExtension<T> {
		const IDENTIFIER: &'static str = "WeightlessExtension";
		type Implicit = ();
		type Pre = ();
		type Val = ();
		fn weight(&self, _call: &CallOf<T>) -> Weight {
			Weight::from_all(0)
		}
		fn validate(
			&self,
			mut origin: <CallOf<T> as Dispatchable>::RuntimeOrigin,
			_: &CallOf<T>,
			_: &DispatchInfoOf<CallOf<T>>,
			_: usize,
			_: (),
			_: &impl Encode,
			_: TransactionSource,
		) -> Result<
			(ValidTransaction, Self::Val, <CallOf<T> as Dispatchable>::RuntimeOrigin),
			TransactionValidityError,
		> {
			origin.set_caller_from_signed(whitelisted_caller());
			Ok((ValidTransaction::default(), (), origin))
		}

		impl_tx_ext_default!(CallOf<T>; prepare);
	}
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	topsoil_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks(
	where
		T: Config,
		<T as Config>::Extension: Default,
	)]
mod benchmarks {
	use codec::Compact;

	use super::*;

	#[benchmark]
	fn bare_dispatch(n: Linear<8, 100>) {
		let meta_call = topsoil_system::Call::<T>::remark { remark: vec![] }.into();
		let meta_ext = T::Extension::default();
		let meta_ext_weight = meta_ext.weight(&meta_call);

		#[cfg(not(test))]
		assert!(
			meta_ext_weight.is_zero(),
			"meta tx extension weight for the benchmarks must be zero. \
			use `plant_meta_tx::WeightlessExtension` as `plant_meta_tx::Config::Extension` \
			with the `runtime-benchmarks` feature enabled.",
		);

		let meta_tx = MetaTxFor::<T>::new(meta_call.clone(), 0u8, meta_ext.clone());

		let caller = whitelisted_caller();
		let origin: <T as topsoil_system::Config>::RuntimeOrigin =
			topsoil_system::RawOrigin::Signed(caller).into();
		let call = Call::<T>::dispatch {
			meta_tx: Box::new(meta_tx.clone()),
			meta_tx_encoded_len: meta_tx.encoded_size() as u32,
		};

		// Encoded size of meta tx is 4 bytes, 4 bytes is size of u16 compact number with max value
		// 0xffff.
		let length_of_compact_vec = ((n - 4) / 4) as usize;
		let mut compact_vec = Vec::<Compact<u16>>::with_capacity(length_of_compact_vec);
		for _ in 0..length_of_compact_vec {
			compact_vec.push(Compact::from(0xffff));
		}

		#[block]
		{
			let _ = compact_vec.encode();
			let _ = call.dispatch_bypass_filter(origin);
		}

		let info = meta_call.get_dispatch_info();
		assert_last_event::<T>(
			Event::Dispatched {
				result: Ok(PostDispatchInfo {
					actual_weight: Some(info.call_weight + meta_ext_weight),
					pays_fee: Pays::Yes,
				}),
			}
			.into(),
		);
	}

	impl_benchmark_test_suite! {
		Pallet,
		crate::mock::new_test_ext(),
		crate::mock::Runtime,
	}
}
