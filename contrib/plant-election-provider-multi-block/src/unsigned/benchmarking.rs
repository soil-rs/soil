// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{
	unsigned::{miner::OffchainWorkerMiner, Call, Config, Pallet},
	verifier::Verifier,
	CurrentPhase, Phase,
};
use subsoil::std::boxed::Box;
use topsoil_benchmarking::v2::*;
use plant_election_provider::ElectionProvider;
use topsoil_support::{assert_ok, pallet_prelude::*};
use topsoil_system::RawOrigin;

#[benchmarks(where T: crate::Config + crate::signed::Config + crate::verifier::Config)]
mod benchmarks {
	use super::*;

	#[benchmark(pov_mode = Measured)]
	fn validate_unsigned() -> Result<(), BenchmarkError> {
		#[cfg(test)]
		crate::mock::ElectionStart::set(subsoil::runtime::traits::Bounded::max_value());
		crate::Pallet::<T>::start().unwrap();

		crate::Pallet::<T>::roll_until_matches(|| {
			matches!(CurrentPhase::<T>::get(), Phase::Unsigned(_))
		});
		let call: Call<T> = OffchainWorkerMiner::<T>::mine_solution(T::MinerPages::get(), false)
			.map(|solution| Call::submit_unsigned { paged_solution: Box::new(solution) })
			.unwrap();

		#[block]
		{
			assert_ok!(Pallet::<T>::validate_unsigned(TransactionSource::Local, &call));
		}

		Ok(())
	}

	#[benchmark(pov_mode = Measured)]
	fn submit_unsigned() -> Result<(), BenchmarkError> {
		#[cfg(test)]
		crate::mock::ElectionStart::set(subsoil::runtime::traits::Bounded::max_value());
		crate::Pallet::<T>::start().unwrap();

		// roll to unsigned phase open
		crate::Pallet::<T>::roll_until_matches(|| {
			matches!(CurrentPhase::<T>::get(), Phase::Unsigned(_))
		});
		// TODO: we need to better ensure that this is actually worst case
		let solution =
			OffchainWorkerMiner::<T>::mine_solution(T::MinerPages::get(), false).unwrap();

		// nothing is queued
		assert!(T::Verifier::queued_score().is_none());
		#[block]
		{
			assert_ok!(Pallet::<T>::submit_unsigned(RawOrigin::None.into(), Box::new(solution)));
		}

		// something is queued
		assert!(T::Verifier::queued_score().is_some());
		Ok(())
	}

	/// NOTE: make sure this benchmark is being run with the correct `type Solver` in `MinerConfig`.
	#[benchmark(extra, pov_mode = Measured)]
	fn mine_solution(p: Linear<1, { T::Pages::get() }>) -> Result<(), BenchmarkError> {
		#[cfg(test)]
		crate::mock::ElectionStart::set(subsoil::runtime::traits::Bounded::max_value());
		crate::Pallet::<T>::start().unwrap();

		// roll to unsigned phase open
		crate::Pallet::<T>::roll_until_matches(|| {
			matches!(CurrentPhase::<T>::get(), Phase::Unsigned(_))
		});

		#[block]
		{
			OffchainWorkerMiner::<T>::mine_solution(p, true).unwrap();
		}

		Ok(())
	}

	impl_benchmark_test_suite!(
		Pallet,
		crate::mock::ExtBuilder::full().build_unchecked(),
		crate::mock::Runtime
	);
}
