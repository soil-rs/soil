// This file is part of Substrate.
//
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

//! Benchmarks for the Session pallet.

#![cfg(feature = "runtime-benchmarks")]

use alloc::vec::Vec;
use crate::{historical::Pallet as Historical, Call, Pallet as Session};
use subsoil::runtime::KeyTypeId;
use topsoil_benchmarking::v2::*;
use topsoil_support::{
	assert_ok,
	traits::{KeyOwnerProofSystem, OnInitialize},
};
use topsoil_system::{pallet_prelude::BlockNumberFor, RawOrigin};

const MAX_VALIDATORS: u32 = 1000;

type MembershipBenchmarkSetup = ((KeyTypeId, Vec<u8>), subsoil::session::MembershipProof);

pub struct Pallet<T: Config>(Session<T>);

/// Runtime-provided helper functions needed to benchmark `plant-session` without introducing
/// benchmark-only dependencies on `plant-staking`.
pub trait Config: crate::Config + crate::historical::Config {
	/// Generate a fresh set of session keys and a valid proof for `owner`.
	fn generate_session_keys_and_proof(owner: Self::AccountId) -> (Self::Keys, Vec<u8>);

	/// Set up a benchmark validator/controller and return the controller account that should submit
	/// the session-key extrinsic.
	fn setup_benchmark_controller() -> Result<Self::AccountId, &'static str>;

	/// Prepare state for the membership-proof benchmarks and return the target key and proof.
	fn setup_membership_proof_benchmark(n: u32) -> Result<MembershipBenchmarkSetup, &'static str>;
}

impl<T: Config> OnInitialize<BlockNumberFor<T>> for Pallet<T> {
	fn on_initialize(n: BlockNumberFor<T>) -> topsoil_support::weights::Weight {
		Session::<T>::on_initialize(n)
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_keys() -> Result<(), BenchmarkError> {
		let controller = T::setup_benchmark_controller().map_err(BenchmarkError::Stop)?;
		let (keys, proof) = T::generate_session_keys_and_proof(controller.clone());
		let controller_key = topsoil_system::Account::<T>::hashed_key_for(&controller);
		topsoil_benchmarking::benchmarking::add_to_whitelist(controller_key.into());
		assert_ok!(Session::<T>::ensure_can_pay_key_deposit(&controller));

		#[extrinsic_call]
		_(RawOrigin::Signed(controller), keys, proof);

		Ok(())
	}

	#[benchmark]
	fn purge_keys() -> Result<(), BenchmarkError> {
		let controller = T::setup_benchmark_controller().map_err(BenchmarkError::Stop)?;
		let (keys, proof) = T::generate_session_keys_and_proof(controller.clone());
		assert_ok!(Session::<T>::ensure_can_pay_key_deposit(&controller));
		Session::<T>::set_keys(RawOrigin::Signed(controller.clone()).into(), keys, proof)?;
		let controller_key = topsoil_system::Account::<T>::hashed_key_for(&controller);
		topsoil_benchmarking::benchmarking::add_to_whitelist(controller_key.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(controller));

		Ok(())
	}

	#[benchmark(extra)]
	fn check_membership_proof_current_session(
		n: Linear<2, MAX_VALIDATORS>,
	) -> Result<(), BenchmarkError> {
		let ((key_type, key_data), proof1) =
			T::setup_membership_proof_benchmark(n).map_err(BenchmarkError::Stop)?;
		let proof2 = proof1.clone();
		let key_for_block = (key_type, key_data.clone());
		let key_for_verify = (key_type, key_data);

		#[block]
		{
			Historical::<T>::check_proof(key_for_block, proof1);
		}

		assert!(Historical::<T>::check_proof(key_for_verify, proof2).is_some());
		Ok(())
	}

	#[benchmark(extra)]
	fn check_membership_proof_historical_session(
		n: Linear<2, MAX_VALIDATORS>,
	) -> Result<(), BenchmarkError> {
		let ((key_type, key_data), proof1) =
			T::setup_membership_proof_benchmark(n).map_err(BenchmarkError::Stop)?;

		Session::<T>::rotate_session();

		let proof2 = proof1.clone();
		let key_for_block = (key_type, key_data.clone());
		let key_for_verify = (key_type, key_data);

		#[block]
		{
			Historical::<T>::check_proof(key_for_block, proof1);
		}

		assert!(Historical::<T>::check_proof(key_for_verify, proof2).is_some());
		Ok(())
	}

	impl_benchmark_test_suite!(
		Pallet,
		crate::benchmarking::mock::new_test_ext(),
		crate::benchmarking::mock::Test,
		extra = false
	);
}

#[cfg(test)]
mod mock;
