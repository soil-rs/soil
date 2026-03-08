// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as TxPause, *};
use alloc::vec;
use topsoil::benchmarking::prelude::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn pause() {
		let origin = T::PauseOrigin::try_successful_origin()
			.expect("Tx-pause pallet is not usable without pause origin");
		let full_name = name::<T>();

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, full_name.clone());

		assert!(PausedCalls::<T>::get(full_name).is_some());
	}

	#[benchmark]
	fn unpause() {
		let unpause_origin = T::UnpauseOrigin::try_successful_origin()
			.expect("Tx-pause pallet is not usable without pause origin");
		let full_name = name::<T>();
		TxPause::<T>::do_pause(full_name.clone()).unwrap();

		#[extrinsic_call]
		_(unpause_origin as T::RuntimeOrigin, full_name.clone());

		assert!(PausedCalls::<T>::get(full_name).is_none());
	}

	impl_benchmark_test_suite!(TxPause, crate::mock::new_test_ext(), crate::mock::Test);
}

/// Longest possible name.
fn name<T: Config>() -> RuntimeCallNameOf<T> {
	let max_len = T::MaxNameLen::get() as usize;
	(vec![1; max_len].try_into().unwrap(), vec![1; max_len].try_into().unwrap())
}
