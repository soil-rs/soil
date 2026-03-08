// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Benchmarks for pallet origin restriction.

use super::*;
use subsoil::runtime::traits::DispatchTransaction;
use topsoil_benchmarking::{v2::*, BenchmarkError};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	topsoil_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benches {
	use super::*;

	#[benchmark]
	fn clean_usage() -> Result<(), BenchmarkError> {
		let origin = T::RestrictedEntity::benchmarked_restricted_origin();
		let entity = T::RestrictedEntity::restricted_entity(&origin)
			.expect("The origin from `benchmarked_restricted_origin` must be restricted");

		Usages::<T>::insert(&entity, Usage { used: 1u32.into(), at_block: 0u32.into() });

		topsoil_system::Pallet::<T>::set_block_number(1_000u32.into());

		#[extrinsic_call]
		_(topsoil_system::RawOrigin::Root, entity.clone());

		assert_last_event::<T>(Event::UsageCleaned { entity }.into());

		Ok(())
	}

	// This benchmark may miss the cost for `OperationAllowedOneTimeExcess::contains`.
	#[benchmark]
	fn restrict_origin_tx_ext() -> Result<(), BenchmarkError> {
		let tx_ext = RestrictOrigin::<T>::new(true);
		let origin = T::RestrictedEntity::benchmarked_restricted_origin();
		let call = topsoil_system::Call::remark { remark: alloc::vec![] }.into();

		#[block]
		{
			tx_ext
				.test_run(origin.into(), &call, &Default::default(), 0, 0, |_| {
					Ok(Default::default())
				})
				.expect("Failed to allow the cheapest call, benchmark needs to be improved")
				.expect("inner call successful");
		}

		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
