// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The Offchain Worker runtime api primitives.

/// Re-export of parent module scope storage prefix.
pub use crate::core::offchain::STORAGE_PREFIX;

crate::api::decl_runtime_apis! {
	/// The offchain worker api.
	#[api_version(2)]
	pub trait OffchainWorkerApi {
		/// Starts the off-chain task for given block number.
		#[changed_in(2)]
		fn offchain_worker(number: crate::runtime::traits::NumberFor<Block>);

		/// Starts the off-chain task for given block header.
		fn offchain_worker(header: &Block::Header);
	}
}
