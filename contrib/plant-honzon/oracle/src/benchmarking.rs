// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) 2020-2025 Acala Foundation.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use crate::Pallet as Oracle;

use topsoil_benchmarking::v2::*;

use topsoil_core::assert_ok;
use topsoil_core::system::{Pallet as System, RawOrigin};

#[instance_benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn feed_values(
		x: Linear<0, { T::BenchmarkHelper::get_currency_id_value_pairs().len() as u32 }>,
	) {
		// Register the caller
		let caller: T::AccountId = whitelisted_caller();
		T::Members::add(&caller);

		let values = T::BenchmarkHelper::get_currency_id_value_pairs()[..x as usize]
			.to_vec()
			.try_into()
			.expect("Must succeed since at worst the length remained the same.");

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), values);

		assert!(HasDispatched::<T, I>::get().contains(&caller));
	}

	#[benchmark]
	fn on_finalize() {
		// Register the caller
		let caller: T::AccountId = whitelisted_caller();
		T::Members::add(&caller);

		// Feed some values before running `on_finalize` hook
		System::<T>::set_block_number(1u32.into());
		let values = T::BenchmarkHelper::get_currency_id_value_pairs();
		assert_ok!(Oracle::<T, I>::feed_values(RawOrigin::Signed(caller).into(), values));

		#[block]
		{
			Oracle::<T, I>::on_finalize(System::<T>::block_number());
		}

		assert!(!HasDispatched::<T, I>::exists());
	}

	impl_benchmark_test_suite! {
		Oracle,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
