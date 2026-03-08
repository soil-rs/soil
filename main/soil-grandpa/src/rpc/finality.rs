// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use serde::{Deserialize, Serialize};

use crate::FinalityProofProvider;
use subsoil::runtime::traits::{Block as BlockT, NumberFor};

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodedFinalityProof(pub subsoil::core::Bytes);

/// Local trait mainly to allow mocking in tests.
pub trait RpcFinalityProofProvider<Block: BlockT> {
	/// Prove finality for the given block number by returning a Justification for the last block of
	/// the authority set.
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProof>, crate::FinalityProofError>;
}

impl<B, Block> RpcFinalityProofProvider<Block> for FinalityProofProvider<B, Block>
where
	Block: BlockT,
	NumberFor<Block>: finality_grandpa::BlockNumberOps,
	B: soil_client::client_api::backend::Backend<Block> + Send + Sync + 'static,
{
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProof>, crate::FinalityProofError> {
		self.prove_finality(block).map(|x| x.map(|y| EncodedFinalityProof(y.into())))
	}
}
