// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! API implementation for the specification of a chain.

use crate::v2::chain_spec::api::ChainSpecApiServer;
use jsonrpsee::core::RpcResult;
use soil_chain_spec::Properties;

/// An API for chain spec RPC calls.
pub struct ChainSpec {
	/// The name of the chain.
	name: String,
	/// The hexadecimal encoded hash of the genesis block.
	genesis_hash: String,
	/// Chain properties.
	properties: Properties,
}

impl ChainSpec {
	/// Creates a new [`ChainSpec`].
	pub fn new<Hash: AsRef<[u8]>>(
		name: String,
		genesis_hash: Hash,
		properties: Properties,
	) -> Self {
		let genesis_hash = format!("0x{}", hex::encode(genesis_hash));

		Self { name, properties, genesis_hash }
	}
}

impl ChainSpecApiServer for ChainSpec {
	fn chain_spec_v1_chain_name(&self) -> RpcResult<String> {
		Ok(self.name.clone())
	}

	fn chain_spec_v1_genesis_hash(&self) -> RpcResult<String> {
		Ok(self.genesis_hash.clone())
	}

	fn chain_spec_v1_properties(&self) -> RpcResult<Properties> {
		Ok(self.properties.clone())
	}
}
