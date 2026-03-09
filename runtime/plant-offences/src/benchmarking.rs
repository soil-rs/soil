// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Benchmarks for the offences pallet.

#![cfg(feature = "runtime-benchmarks")]

use alloc::vec::Vec;
use crate::Pallet as Offences;
use subsoil::staking::offence::{Offence, ReportOffence};
use topsoil_benchmarking::v2::*;
use topsoil_core::traits::Get;

const MAX_NOMINATORS: u32 = 100;

pub struct Pallet<T: Config>(Offences<T>);

/// Runtime-provided helper functions needed to benchmark `plant-offences` without introducing
/// benchmark-only dependencies on BABE, GRANDPA, IM Online, Session, or Staking.
pub trait Config: crate::Config {
	type MaxNominators: Get<u32>;
	type BabeBenchmarkOffence: Offence<Self::IdentificationTuple>;
	type GrandpaBenchmarkOffence: Offence<Self::IdentificationTuple>;

	fn setup_babe_benchmark(
		n: u32,
	) -> Result<(Vec<Self::AccountId>, Self::BabeBenchmarkOffence), &'static str>;

	fn setup_grandpa_benchmark(
		n: u32,
	) -> Result<(Vec<Self::AccountId>, Self::GrandpaBenchmarkOffence), &'static str>;

	#[cfg(test)]
	fn assert_all_slashes_applied(_offender_count: usize) {}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	pub fn report_offence_grandpa(
		n: Linear<0, { MAX_NOMINATORS.min(T::MaxNominators::get()) }>,
	) -> Result<(), BenchmarkError> {
		let (reporters, offence) = T::setup_grandpa_benchmark(n).map_err(BenchmarkError::Stop)?;

		#[block]
		{
			let _ = Offences::<T>::report_offence(reporters, offence);
		}

		#[cfg(test)]
		T::assert_all_slashes_applied(n as usize);

		Ok(())
	}

	#[benchmark]
	fn report_offence_babe(
		n: Linear<0, { MAX_NOMINATORS.min(T::MaxNominators::get()) }>,
	) -> Result<(), BenchmarkError> {
		let (reporters, offence) = T::setup_babe_benchmark(n).map_err(BenchmarkError::Stop)?;

		#[block]
		{
			let _ = Offences::<T>::report_offence(reporters, offence);
		}

		#[cfg(test)]
		T::assert_all_slashes_applied(n as usize);

		Ok(())
	}

}
