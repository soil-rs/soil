// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Block Builder extensions for tests.

use soil_client::block_builder::BlockBuilderApi;
use subsoil::api::{ApiExt, ProvideRuntimeApi};
use soil_test_node_runtime::*;

/// Extension trait for test block builder.
pub trait BlockBuilderExt {
	/// Add transfer extrinsic to the block.
	fn push_transfer(
		&mut self,
		transfer: soil_test_node_runtime::Transfer,
	) -> Result<(), soil_client::blockchain::Error>;

	/// Add unsigned storage change extrinsic to the block.
	fn push_storage_change(
		&mut self,
		key: Vec<u8>,
		value: Option<Vec<u8>>,
	) -> Result<(), soil_client::blockchain::Error>;

	/// Adds an extrinsic which pushes DigestItem to header's log
	fn push_deposit_log_digest_item(
		&mut self,
		log: subsoil::runtime::generic::DigestItem,
	) -> Result<(), soil_client::blockchain::Error>;
}

impl<'a, A> BlockBuilderExt
	for soil_client::block_builder::BlockBuilder<'a, soil_test_node_runtime::Block, A>
where
	A: ProvideRuntimeApi<soil_test_node_runtime::Block>
		+ subsoil::api::CallApiAt<soil_test_node_runtime::Block>
		+ 'a,
	A::Api: BlockBuilderApi<soil_test_node_runtime::Block> + ApiExt<soil_test_node_runtime::Block>,
{
	fn push_transfer(
		&mut self,
		transfer: soil_test_node_runtime::Transfer,
	) -> Result<(), soil_client::blockchain::Error> {
		self.push(transfer.into_unchecked_extrinsic())
	}

	fn push_storage_change(
		&mut self,
		key: Vec<u8>,
		value: Option<Vec<u8>>,
	) -> Result<(), soil_client::blockchain::Error> {
		self.push(ExtrinsicBuilder::new_storage_change(key, value).build())
	}

	fn push_deposit_log_digest_item(
		&mut self,
		log: subsoil::runtime::generic::DigestItem,
	) -> Result<(), soil_client::blockchain::Error> {
		self.push(ExtrinsicBuilder::new_deposit_log_digest_item(log).build())
	}
}
