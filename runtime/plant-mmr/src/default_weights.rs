// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Default weights for the MMR Pallet
//! This file was not auto-generated.

use topsoil::{deps::topsoil_support::weights::constants::*, weights_prelude::*};

impl crate::WeightInfo for () {
	fn on_initialize(peaks: u32) -> Weight {
		let peaks = u64::from(peaks);
		// Reading the parent hash.
		let leaf_weight = RocksDbWeight::get().reads(1);
		// Blake2 hash cost.
		let hash_weight = Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_NANOS, 0);
		// No-op hook.
		let hook_weight = Weight::zero();

		leaf_weight
			.saturating_add(hash_weight)
			.saturating_add(hook_weight)
			.saturating_add(RocksDbWeight::get().reads_writes(2 + peaks, 2 + peaks))
	}
}
