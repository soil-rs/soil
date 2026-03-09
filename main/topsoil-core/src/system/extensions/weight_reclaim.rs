// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::system::Config;
use codec::{Decode, DecodeWithMemTracking, Encode};
use scale_info::TypeInfo;
use subsoil::runtime::{
	traits::{
		DispatchInfoOf, Dispatchable, PostDispatchInfoOf, TransactionExtension, ValidateResult,
	},
	transaction_validity::{TransactionSource, TransactionValidityError, ValidTransaction},
	DispatchResult,
};
use subsoil::weights::Weight;
use topsoil_core::dispatch::{DispatchInfo, PostDispatchInfo};

/// Reclaim the unused weight using the post dispatch information
///
/// After the dispatch of the extrinsic, calculate the unused weight using the post dispatch
/// information and update the block consumed weight according to the new calculated extrinsic
/// weight.
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, Default, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct WeightReclaim<T: Config + Send + Sync>(core::marker::PhantomData<T>);

impl<T: Config + Send + Sync> WeightReclaim<T>
where
	T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	/// Creates new `TransactionExtension` to recalculate the extrinsic weight after dispatch.
	pub fn new() -> Self {
		Self(Default::default())
	}
}

impl<T: Config + Send + Sync> TransactionExtension<T::RuntimeCall> for WeightReclaim<T>
where
	T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	const IDENTIFIER: &'static str = "WeightReclaim";
	type Implicit = ();
	type Pre = ();
	type Val = ();

	fn weight(&self, _: &T::RuntimeCall) -> Weight {
		<T::ExtensionsWeightInfo as super::WeightInfo>::weight_reclaim()
	}

	fn validate(
		&self,
		origin: T::RuntimeOrigin,
		_call: &T::RuntimeCall,
		_info: &DispatchInfoOf<T::RuntimeCall>,
		_len: usize,
		_self_implicit: Self::Implicit,
		_inherited_implication: &impl Encode,
		_source: TransactionSource,
	) -> ValidateResult<Self::Val, T::RuntimeCall> {
		Ok((ValidTransaction::default(), (), origin))
	}

	fn prepare(
		self,
		_val: Self::Val,
		_origin: &T::RuntimeOrigin,
		_call: &T::RuntimeCall,
		_info: &DispatchInfoOf<T::RuntimeCall>,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Ok(())
	}

	fn post_dispatch_details(
		_pre: Self::Pre,
		info: &DispatchInfoOf<T::RuntimeCall>,
		post_info: &PostDispatchInfoOf<T::RuntimeCall>,
		_len: usize,
		_result: &DispatchResult,
	) -> Result<Weight, TransactionValidityError> {
		crate::system::Pallet::<T>::reclaim_weight(info, post_info).map(|()| Weight::zero())
	}

	fn bare_validate(
		_call: &T::RuntimeCall,
		_info: &DispatchInfoOf<T::RuntimeCall>,
		_len: usize,
	) -> topsoil_core::pallet_prelude::TransactionValidity {
		Ok(ValidTransaction::default())
	}

	fn bare_validate_and_prepare(
		_call: &T::RuntimeCall,
		_info: &DispatchInfoOf<T::RuntimeCall>,
		_len: usize,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn bare_post_dispatch(
		info: &DispatchInfoOf<T::RuntimeCall>,
		post_info: &mut PostDispatchInfoOf<T::RuntimeCall>,
		_len: usize,
		_result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		crate::system::Pallet::<T>::reclaim_weight(info, post_info)
	}
}

impl<T: Config + Send + Sync> core::fmt::Debug for WeightReclaim<T>
where
	T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
{
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", Self::IDENTIFIER)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::system::{
		mock::{new_test_ext, Test},
		BlockWeight, DispatchClass,
	};
	use topsoil_core::{assert_ok, weights::Weight};

	fn block_weights() -> crate::system::limits::BlockWeights {
		<Test as crate::system::Config>::BlockWeights::get()
	}

	#[test]
	fn extrinsic_already_refunded_more_precisely() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info =
				DispatchInfo { call_weight: Weight::from_parts(512, 0), ..Default::default() };
			let post_info = PostDispatchInfo {
				actual_weight: Some(Weight::from_parts(128, 0)),
				pays_fee: Default::default(),
			};
			let prior_block_weight = Weight::from_parts(64, 0);
			let accurate_refund = Weight::from_parts(510, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Normal).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Normal);
				current_weight.accrue(
					base_extrinsic + info.total_weight() - accurate_refund,
					DispatchClass::Normal,
				);
			});
			crate::system::ExtrinsicWeightReclaimed::<Test>::put(accurate_refund);

			// Do the post dispatch
			assert_ok!(WeightReclaim::<Test>::post_dispatch_details(
				(),
				&info,
				&post_info,
				len,
				&Ok(())
			));

			// Ensure the accurate refund is used
			assert_eq!(crate::system::ExtrinsicWeightReclaimed::<Test>::get(), accurate_refund);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Normal),
				info.total_weight() - accurate_refund + prior_block_weight + base_extrinsic
			);
		})
	}

	#[test]
	fn extrinsic_already_refunded_less_precisely() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info =
				DispatchInfo { call_weight: Weight::from_parts(512, 0), ..Default::default() };
			let post_info = PostDispatchInfo {
				actual_weight: Some(Weight::from_parts(128, 0)),
				pays_fee: Default::default(),
			};
			let prior_block_weight = Weight::from_parts(64, 0);
			let inaccurate_refund = Weight::from_parts(110, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Normal).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Normal);
				current_weight.accrue(
					base_extrinsic + info.total_weight() - inaccurate_refund,
					DispatchClass::Normal,
				);
			});
			crate::system::ExtrinsicWeightReclaimed::<Test>::put(inaccurate_refund);

			// Do the post dispatch
			assert_ok!(WeightReclaim::<Test>::post_dispatch_details(
				(),
				&info,
				&post_info,
				len,
				&Ok(())
			));

			// Ensure the accurate refund from benchmark is used
			assert_eq!(
				crate::system::ExtrinsicWeightReclaimed::<Test>::get(),
				post_info.calc_unspent(&info)
			);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Normal),
				post_info.actual_weight.unwrap() + prior_block_weight + base_extrinsic
			);
		})
	}

	#[test]
	fn extrinsic_not_refunded_before() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info =
				DispatchInfo { call_weight: Weight::from_parts(512, 0), ..Default::default() };
			let post_info = PostDispatchInfo {
				actual_weight: Some(Weight::from_parts(128, 0)),
				pays_fee: Default::default(),
			};
			let prior_block_weight = Weight::from_parts(64, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Normal).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Normal);
				current_weight.accrue(base_extrinsic + info.total_weight(), DispatchClass::Normal);
			});

			// Do the post dispatch
			assert_ok!(WeightReclaim::<Test>::post_dispatch_details(
				(),
				&info,
				&post_info,
				len,
				&Ok(())
			));

			// Ensure the accurate refund from benchmark is used
			assert_eq!(
				crate::system::ExtrinsicWeightReclaimed::<Test>::get(),
				post_info.calc_unspent(&info)
			);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Normal),
				post_info.actual_weight.unwrap() + prior_block_weight + base_extrinsic
			);
		})
	}

	#[test]
	fn no_actual_post_dispatch_weight() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info =
				DispatchInfo { call_weight: Weight::from_parts(512, 0), ..Default::default() };
			let post_info = PostDispatchInfo { actual_weight: None, pays_fee: Default::default() };
			let prior_block_weight = Weight::from_parts(64, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Normal).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Normal);
				current_weight.accrue(base_extrinsic + info.total_weight(), DispatchClass::Normal);
			});

			// Do the post dispatch
			assert_ok!(WeightReclaim::<Test>::post_dispatch_details(
				(),
				&info,
				&post_info,
				len,
				&Ok(())
			));

			// Ensure the accurate refund from benchmark is used
			assert_eq!(
				crate::system::ExtrinsicWeightReclaimed::<Test>::get(),
				post_info.calc_unspent(&info)
			);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Normal),
				info.total_weight() + prior_block_weight + base_extrinsic
			);
		})
	}

	#[test]
	fn different_dispatch_class() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info = DispatchInfo {
				call_weight: Weight::from_parts(512, 0),
				class: DispatchClass::Operational,
				..Default::default()
			};
			let post_info = PostDispatchInfo {
				actual_weight: Some(Weight::from_parts(128, 0)),
				pays_fee: Default::default(),
			};
			let prior_block_weight = Weight::from_parts(64, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Operational).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Operational);
				current_weight
					.accrue(base_extrinsic + info.total_weight(), DispatchClass::Operational);
			});

			// Do the post dispatch
			assert_ok!(WeightReclaim::<Test>::post_dispatch_details(
				(),
				&info,
				&post_info,
				len,
				&Ok(())
			));

			// Ensure the accurate refund from benchmark is used
			assert_eq!(
				crate::system::ExtrinsicWeightReclaimed::<Test>::get(),
				post_info.calc_unspent(&info)
			);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Operational),
				post_info.actual_weight.unwrap() + prior_block_weight + base_extrinsic
			);
		})
	}

	#[test]
	fn bare_also_works() {
		new_test_ext().execute_with(|| {
			// This is half of the max block weight
			let info = DispatchInfo {
				call_weight: Weight::from_parts(512, 0),
				class: DispatchClass::Operational,
				..Default::default()
			};
			let post_info = PostDispatchInfo {
				actual_weight: Some(Weight::from_parts(128, 0)),
				pays_fee: Default::default(),
			};
			let prior_block_weight = Weight::from_parts(64, 0);
			let len = 0_usize;
			let base_extrinsic = block_weights().get(DispatchClass::Operational).base_extrinsic;

			// Set initial info
			BlockWeight::<Test>::mutate(|current_weight| {
				current_weight.set(prior_block_weight, DispatchClass::Operational);
				current_weight
					.accrue(base_extrinsic + info.total_weight(), DispatchClass::Operational);
			});

			// Do the bare post dispatch
			assert_ok!(WeightReclaim::<Test>::bare_post_dispatch(
				&info,
				&mut post_info.clone(),
				len,
				&Ok(())
			));

			// Ensure the accurate refund from benchmark is used
			assert_eq!(
				crate::system::ExtrinsicWeightReclaimed::<Test>::get(),
				post_info.calc_unspent(&info)
			);
			assert_eq!(
				*BlockWeight::<Test>::get().get(DispatchClass::Operational),
				post_info.actual_weight.unwrap() + prior_block_weight + base_extrinsic
			);
		})
	}
}
