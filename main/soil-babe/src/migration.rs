// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{
	AuthorityId, BabeAuthorityWeight, BabeConfiguration, BabeEpochConfiguration, Epoch,
	NextEpochDescriptor, Randomness,
};
use codec::{Decode, Encode};
use soil_consensus::epochs::Epoch as EpochT;
use subsoil::consensus::slots::Slot;

/// BABE epoch information, version 0.
#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct EpochV0 {
	/// The epoch index.
	pub epoch_index: u64,
	/// The starting slot of the epoch.
	pub start_slot: Slot,
	/// The duration of this epoch.
	pub duration: u64,
	/// The authorities and their weights.
	pub authorities: Vec<(AuthorityId, BabeAuthorityWeight)>,
	/// Randomness for this epoch.
	pub randomness: Randomness,
}

impl EpochT for EpochV0 {
	type NextEpochDescriptor = NextEpochDescriptor;
	type Slot = Slot;

	fn increment(&self, descriptor: NextEpochDescriptor) -> EpochV0 {
		EpochV0 {
			epoch_index: self.epoch_index + 1,
			start_slot: self.start_slot + self.duration,
			duration: self.duration,
			authorities: descriptor.authorities,
			randomness: descriptor.randomness,
		}
	}

	fn start_slot(&self) -> Slot {
		self.start_slot
	}

	fn end_slot(&self) -> Slot {
		self.start_slot + self.duration
	}
}

// Implement From<EpochV0> for Epoch
impl EpochV0 {
	/// Migrate the struct to current epoch version.
	pub fn migrate(self, config: &BabeConfiguration) -> Epoch {
		subsoil::consensus::babe::Epoch {
			epoch_index: self.epoch_index,
			start_slot: self.start_slot,
			duration: self.duration,
			authorities: self.authorities,
			randomness: self.randomness,
			config: BabeEpochConfiguration { c: config.c, allowed_slots: config.allowed_slots },
		}
		.into()
	}
}
