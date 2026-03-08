// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(feature = "runtime-benchmarks")]

use alloc::vec;
use subsoil::runtime::{
	generic::Era,
	traits::{
		AsSystemOriginSigner, AsTransactionAuthorizedOrigin, DispatchTransaction, Dispatchable, Get,
	},
};
use topsoil_benchmarking::{account, v2::*, BenchmarkError};
use topsoil_support::{
	dispatch::{DispatchClass, DispatchInfo, PostDispatchInfo},
	pallet_prelude::Zero,
	weights::Weight,
};
use topsoil_system::{
	pallet_prelude::*, CheckGenesis, CheckMortality, CheckNonZeroSender, CheckNonce,
	CheckSpecVersion, CheckTxVersion, CheckWeight, Config, ExtensionsWeightInfo, Pallet as System,
	RawOrigin, WeightReclaim,
};

pub struct Pallet<T: Config>(System<T>);

#[benchmarks(where
	T: Send + Sync,
    T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	<T::RuntimeCall as Dispatchable>::RuntimeOrigin: AsSystemOriginSigner<T::AccountId> + AsTransactionAuthorizedOrigin + Clone,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn check_genesis() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let caller = account("caller", 0, 0);
		let info = DispatchInfo { call_weight: Weight::zero(), ..Default::default() };
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();
		topsoil_benchmarking::benchmarking::add_to_whitelist(
			topsoil_system::BlockHash::<T>::hashed_key_for(BlockNumberFor::<T>::zero()).into(),
		);

		#[block]
		{
			CheckGenesis::<T>::new()
				.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}

		Ok(())
	}

	#[benchmark]
	fn check_mortality_mortal_transaction() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let ext = CheckMortality::<T>::from(Era::mortal(16, 256));
		let block_number: BlockNumberFor<T> = 17u32.into();
		System::<T>::set_block_number(block_number);
		let prev_block: BlockNumberFor<T> = 16u32.into();
		let default_hash: T::Hash = Default::default();
		topsoil_system::BlockHash::<T>::insert(prev_block, default_hash);
		let caller = account("caller", 0, 0);
		let info = DispatchInfo {
			call_weight: Weight::from_parts(100, 0),
			class: DispatchClass::Normal,
			..Default::default()
		};
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();
		topsoil_benchmarking::benchmarking::add_to_whitelist(
			topsoil_system::BlockHash::<T>::hashed_key_for(prev_block).into(),
		);

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}
		Ok(())
	}

	#[benchmark]
	fn check_mortality_immortal_transaction() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let ext = CheckMortality::<T>::from(Era::immortal());
		let block_number: BlockNumberFor<T> = 17u32.into();
		System::<T>::set_block_number(block_number);
		let prev_block: BlockNumberFor<T> = 16u32.into();
		let default_hash: T::Hash = Default::default();
		topsoil_system::BlockHash::<T>::insert(prev_block, default_hash);
		let genesis_block: BlockNumberFor<T> = 0u32.into();
		topsoil_system::BlockHash::<T>::insert(genesis_block, default_hash);
		let caller = account("caller", 0, 0);
		let info = DispatchInfo {
			call_weight: Weight::from_parts(100, 0),
			class: DispatchClass::Normal,
			..Default::default()
		};
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();
		topsoil_benchmarking::benchmarking::add_to_whitelist(
			topsoil_system::BlockHash::<T>::hashed_key_for(BlockNumberFor::<T>::zero()).into(),
		);

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}
		Ok(())
	}

	#[benchmark]
	fn check_non_zero_sender() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let ext = CheckNonZeroSender::<T>::new();
		let caller = account("caller", 0, 0);
		let info = DispatchInfo { call_weight: Weight::zero(), ..Default::default() };
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}
		Ok(())
	}

	#[benchmark]
	fn check_nonce() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = account("caller", 0, 0);
		let mut info = topsoil_system::AccountInfo::default();
		info.nonce = 1u32.into();
		info.providers = 1;
		let expected_nonce = info.nonce + 1u32.into();
		topsoil_system::Account::<T>::insert(caller.clone(), info);
		let len = 0_usize;
		let ext = CheckNonce::<T>::from(1u32.into());
		let info = DispatchInfo { call_weight: Weight::zero(), ..Default::default() };
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller.clone()).into(), &call, &info, len, 0, |_| {
				Ok(().into())
			})
			.unwrap()
			.unwrap();
		}

		let updated_info = topsoil_system::Account::<T>::get(caller.clone());
		assert_eq!(updated_info.nonce, expected_nonce);
		Ok(())
	}

	#[benchmark]
	fn check_spec_version() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let caller = account("caller", 0, 0);
		let info = DispatchInfo { call_weight: Weight::zero(), ..Default::default() };
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();

		#[block]
		{
			CheckSpecVersion::<T>::new()
				.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}
		Ok(())
	}

	#[benchmark]
	fn check_tx_version() -> Result<(), BenchmarkError> {
		let len = 0_usize;
		let caller = account("caller", 0, 0);
		let info = DispatchInfo { call_weight: Weight::zero(), ..Default::default() };
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();

		#[block]
		{
			CheckTxVersion::<T>::new()
				.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(().into()))
				.unwrap()
				.unwrap();
		}
		Ok(())
	}

	#[benchmark]
	fn check_weight() -> Result<(), BenchmarkError> {
		let caller = account("caller", 0, 0);
		let base_extrinsic = <T as topsoil_system::Config>::BlockWeights::get()
			.get(DispatchClass::Normal)
			.base_extrinsic;
		let extension_weight = <T as topsoil_system::Config>::ExtensionsWeightInfo::check_weight();
		let info = DispatchInfo {
			call_weight: Weight::from_parts(base_extrinsic.ref_time() * 5, 0),
			extension_weight,
			class: DispatchClass::Normal,
			..Default::default()
		};
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();
		let post_info = PostDispatchInfo {
			actual_weight: Some(Weight::from_parts(base_extrinsic.ref_time() * 2, 0)),
			pays_fee: Default::default(),
		};
		let len = 0_usize;
		let base_extrinsic = <T as topsoil_system::Config>::BlockWeights::get()
			.get(DispatchClass::Normal)
			.base_extrinsic;

		let ext = CheckWeight::<T>::new();

		let initial_block_weight = Weight::from_parts(base_extrinsic.ref_time() * 2, 0);
		topsoil_system::BlockWeight::<T>::mutate(|current_weight| {
			current_weight.set(Weight::zero(), DispatchClass::Mandatory);
			current_weight.set(initial_block_weight, DispatchClass::Normal);
		});

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(post_info))
				.unwrap()
				.unwrap();
		}

		assert_eq!(
			System::<T>::block_weight().total(),
			initial_block_weight
				+ base_extrinsic
				+ post_info.actual_weight.unwrap().saturating_add(extension_weight),
		);
		Ok(())
	}

	#[benchmark]
	fn weight_reclaim() -> Result<(), BenchmarkError> {
		let caller = account("caller", 0, 0);
		let base_extrinsic = <T as topsoil_system::Config>::BlockWeights::get()
			.get(DispatchClass::Normal)
			.base_extrinsic;
		let extension_weight =
			<T as topsoil_system::Config>::ExtensionsWeightInfo::weight_reclaim();
		let info = DispatchInfo {
			call_weight: Weight::from_parts(base_extrinsic.ref_time() * 5, 0),
			extension_weight,
			class: DispatchClass::Normal,
			..Default::default()
		};
		let call: T::RuntimeCall = topsoil_system::Call::remark { remark: vec![] }.into();
		let post_info = PostDispatchInfo {
			actual_weight: Some(Weight::from_parts(base_extrinsic.ref_time() * 2, 0)),
			pays_fee: Default::default(),
		};
		let len = 0_usize;
		let ext = WeightReclaim::<T>::new();

		let initial_block_weight = Weight::from_parts(base_extrinsic.ref_time() * 2, 0);
		topsoil_system::BlockWeight::<T>::mutate(|current_weight| {
			current_weight.set(Weight::zero(), DispatchClass::Mandatory);
			current_weight.set(initial_block_weight, DispatchClass::Normal);
			current_weight.accrue(base_extrinsic + info.total_weight(), DispatchClass::Normal);
		});

		#[block]
		{
			ext.test_run(RawOrigin::Signed(caller).into(), &call, &info, len, 0, |_| Ok(post_info))
				.unwrap()
				.unwrap();
		}

		assert_eq!(
			System::<T>::block_weight().total(),
			initial_block_weight
				+ base_extrinsic
				+ post_info.actual_weight.unwrap().saturating_add(extension_weight),
		);
		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
}
