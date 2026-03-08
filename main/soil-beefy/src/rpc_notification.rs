// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use codec::Encode;
use serde::{Deserialize, Serialize};

use subsoil::consensus::beefy::AuthorityIdBound;
use subsoil::runtime::traits::Block as BlockT;

/// An encoded finality proof proving that the given header has been finalized.
/// The given bytes should be the SCALE-encoded representation of a
/// `subsoil::consensus::beefy::VersionedFinalityProof`.
#[derive(Clone, Serialize, Deserialize)]
pub struct EncodedVersionedFinalityProof(subsoil::core::Bytes);

impl EncodedVersionedFinalityProof {
	pub fn new<Block, AuthorityId>(
		finality_proof: crate::justification::BeefyVersionedFinalityProof<Block, AuthorityId>,
	) -> Self
	where
		Block: BlockT,
		AuthorityId: AuthorityIdBound,
	{
		EncodedVersionedFinalityProof(finality_proof.encode().into())
	}
}
