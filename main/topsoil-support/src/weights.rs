// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Re-exports `sp-weights` public API, and contains benchmarked weight constants specific to FRAME.

mod block_weights;
mod extrinsic_weights;
mod paritydb_weights;
mod rocksdb_weights;

pub use subsoil::weights::*;

/// These constants are specific to FRAME, and the current implementation of its various components.
/// For example: FRAME System, FRAME Executive, our FRAME support libraries, etc...
pub mod constants {
	pub use subsoil::weights::constants::*;

	// Expose the Block and Extrinsic base weights.
	pub use super::{block_weights::BlockExecutionWeight, extrinsic_weights::ExtrinsicBaseWeight};

	// Expose the DB weights.
	pub use super::{
		paritydb_weights::constants::ParityDbWeight, rocksdb_weights::constants::RocksDbWeight,
	};
}
