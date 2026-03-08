// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Default weights for the BEEFY Pallet
//! This file was not auto-generated.

use topsoil_support::weights::{
	constants::{RocksDbWeight as DbWeight, WEIGHT_REF_TIME_PER_MICROS, WEIGHT_REF_TIME_PER_NANOS},
	Weight,
};

impl crate::WeightInfo for () {
	fn report_voting_equivocation(
		votes_count: u32,
		validator_count: u32,
		max_nominators_per_validator: u32,
	) -> Weight {
		// we take the validator set count from the membership proof to
		// calculate the weight but we set a floor of 100 validators.
		let validator_count = validator_count.max(100) as u64;

		// checking membership proof
		Weight::from_parts(35u64 * WEIGHT_REF_TIME_PER_MICROS, 0)
			.saturating_add(
				Weight::from_parts(175u64 * WEIGHT_REF_TIME_PER_NANOS, 0)
					.saturating_mul(validator_count),
			)
			.saturating_add(DbWeight::get().reads(5))
			// check equivocation proof
			.saturating_add(Weight::from_parts(
				(50u64 * WEIGHT_REF_TIME_PER_MICROS).saturating_mul(votes_count as u64),
				0,
			))
			// report offence
			.saturating_add(Weight::from_parts(110u64 * WEIGHT_REF_TIME_PER_MICROS, 0))
			.saturating_add(Weight::from_parts(
				25u64 * WEIGHT_REF_TIME_PER_MICROS * max_nominators_per_validator as u64,
				0,
			))
			.saturating_add(DbWeight::get().reads(14 + 3 * max_nominators_per_validator as u64))
			.saturating_add(DbWeight::get().writes(10 + 3 * max_nominators_per_validator as u64))
			// fetching set id -> session index mappings
			.saturating_add(DbWeight::get().reads(2))
	}

	fn set_new_genesis() -> Weight {
		DbWeight::get().writes(1)
	}
}
