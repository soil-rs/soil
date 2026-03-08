// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Timestamp pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use subsoil::storage::TrackedStorageKey;
use topsoil_benchmarking::{benchmarking::add_to_whitelist, v2::*};
use topsoil_support::traits::OnFinalize;
use topsoil_system::RawOrigin;

use crate::*;

const MAX_TIME: u32 = 100;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set() {
		let t = MAX_TIME;
		// Ignore write to `DidUpdate` since it transient.
		let did_update_key = DidUpdate::<T>::hashed_key().to_vec();
		add_to_whitelist(TrackedStorageKey {
			key: did_update_key,
			reads: 0,
			writes: 1,
			whitelisted: false,
		});

		#[extrinsic_call]
		_(RawOrigin::None, t.into());

		assert_eq!(Now::<T>::get(), t.into(), "Time was not set.");
	}

	#[benchmark]
	fn on_finalize() {
		let t = MAX_TIME;
		Pallet::<T>::set(RawOrigin::None.into(), t.into()).unwrap();
		assert!(DidUpdate::<T>::exists(), "Time was not set.");

		// Ignore read/write to `DidUpdate` since it is transient.
		let did_update_key = DidUpdate::<T>::hashed_key().to_vec();
		add_to_whitelist(did_update_key.into());

		#[block]
		{
			Pallet::<T>::on_finalize(t.into());
		}

		assert!(!DidUpdate::<T>::exists(), "Time was not removed.");
	}

	impl_benchmark_test_suite! {
		Pallet,
		mock::new_test_ext(),
		mock::Test
	}
}
