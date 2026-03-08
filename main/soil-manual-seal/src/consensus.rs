// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Extensions for manual seal to produce blocks valid for any runtime.
use super::Error;

use soil_client::import::BlockImportParams;
use subsoil::api::StorageProof;
use subsoil::inherents::InherentData;
use subsoil::runtime::{traits::Block as BlockT, Digest};

pub mod aura;
pub mod babe;
pub mod timestamp;

/// Consensus data provider, manual seal uses this trait object for authoring blocks valid
/// for any runtime.
pub trait ConsensusDataProvider<B: BlockT>: Send + Sync {
	/// Attempt to create a consensus digest.
	fn create_digest(&self, parent: &B::Header, inherents: &InherentData) -> Result<Digest, Error>;

	/// Set up the necessary import params.
	fn append_block_import(
		&self,
		parent: &B::Header,
		params: &mut BlockImportParams<B>,
		inherents: &InherentData,
		proof: StorageProof,
	) -> Result<(), Error>;
}
