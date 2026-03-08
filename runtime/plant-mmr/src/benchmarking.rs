// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Benchmarks for the MMR pallet.

#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use topsoil::{
	benchmarking::prelude::v1::benchmarks_instance_pallet,
	deps::topsoil_support::traits::OnInitialize,
};

benchmarks_instance_pallet! {
	on_initialize {
		let x in 1 .. 1_000;

		let leaves = x as NodeIndex;

		<<T as pallet::Config::<I>>::BenchmarkHelper as BenchmarkHelper>::setup();
		for leaf in 0..(leaves - 1) {
			<Pallet::<T, I> as OnInitialize<BlockNumberFor<T>>>::on_initialize((leaf as u32).into());
		}
	}: {
		<Pallet::<T, I> as OnInitialize<BlockNumberFor<T>>>::on_initialize((leaves as u32 - 1).into());
	} verify {
		assert_eq!(crate::NumberOfLeaves::<T, I>::get(), leaves);
	}

	impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::mock::Test);
}
