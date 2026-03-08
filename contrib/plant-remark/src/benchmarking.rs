// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Benchmarks for remarks pallet

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use alloc::vec;
use topsoil_benchmarking::v2::*;
use topsoil_system::{EventRecord, Pallet as System, RawOrigin};

#[cfg(test)]
use crate::Pallet as Remark;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = System::<T>::events();
	let system_event: <T as topsoil_system::Config>::RuntimeEvent = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn store(l: Linear<1, { 1024 * 1024 }>) {
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), vec![0u8; l as usize]);

		assert_last_event::<T>(
			Event::Stored {
				sender: caller,
				content_hash: subsoil::io::hashing::blake2_256(&vec![0u8; l as usize]).into(),
			}
			.into(),
		);
	}

	impl_benchmark_test_suite!(Remark, crate::mock::new_test_ext(), crate::mock::Test);
}
