// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Primitives for Substrate Proof-of-Work (PoW) consensus.

use crate::runtime::ConsensusEngineId;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use codec::Decode;

/// The `ConsensusEngineId` of PoW.
pub const POW_ENGINE_ID: ConsensusEngineId = [b'p', b'o', b'w', b'_'];

/// Type of seal.
pub type Seal = Vec<u8>;

/// Define methods that total difficulty should implement.
pub trait TotalDifficulty {
	fn increment(&mut self, other: Self);
}

impl TotalDifficulty for crate::core::U256 {
	fn increment(&mut self, other: Self) {
		let ret = self.saturating_add(other);
		*self = ret;
	}
}

impl TotalDifficulty for u128 {
	fn increment(&mut self, other: Self) {
		let ret = self.saturating_add(other);
		*self = ret;
	}
}

crate::api::decl_runtime_apis! {
	/// API necessary for timestamp-based difficulty adjustment algorithms.
	pub trait TimestampApi<Moment: Decode> {
		/// Return the timestamp in the current block.
		fn timestamp() -> Moment;
	}

	/// API for those chains that put their difficulty adjustment algorithm directly
	/// onto runtime. Note that while putting difficulty adjustment algorithm to
	/// runtime is safe, putting the PoW algorithm on runtime is not.
	pub trait DifficultyApi<Difficulty: Decode> {
		/// Return the target difficulty of the next block.
		fn difficulty() -> Difficulty;
	}
}
