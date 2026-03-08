// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Abstract interfaces and data structures related to network sync.

pub mod message;

/// Sync operation mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyncMode {
	/// Full block download and verification.
	Full,
	/// Download blocks and the latest state.
	LightState {
		/// Skip state proof download and verification.
		skip_proofs: bool,
		/// Download indexed transactions for recent blocks.
		storage_chain_mode: bool,
	},
	/// Warp sync - verify authority set transitions and the latest state.
	Warp,
}

impl SyncMode {
	/// Returns `true` if `self` is [`Self::Warp`].
	pub fn is_warp(&self) -> bool {
		matches!(self, Self::Warp)
	}

	/// Returns `true` if `self` is [`Self::LightState`].
	pub fn light_state(&self) -> bool {
		matches!(self, Self::LightState { .. })
	}
}

impl Default for SyncMode {
	fn default() -> Self {
		Self::Full
	}
}
