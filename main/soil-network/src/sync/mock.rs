// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Contains mock implementations of `ChainSync` and 'BlockDownloader'.

use crate::sync::block_relay_protocol::{BlockDownloader as BlockDownloaderT, BlockResponseError};

use futures::channel::oneshot;
use soil_network::common::sync::message::{BlockData, BlockRequest};
use soil_network::types::PeerId;
use soil_network::{ProtocolName, RequestFailure};
use subsoil::runtime::traits::Block as BlockT;

mockall::mock! {
	#[derive(Debug)]
	pub BlockDownloader<Block: BlockT> {}

	#[async_trait::async_trait]
	impl<Block: BlockT> BlockDownloaderT<Block> for BlockDownloader<Block> {
		fn protocol_name(&self) -> &ProtocolName;

		async fn download_blocks(
			&self,
			who: PeerId,
			request: BlockRequest<Block>,
		) -> Result<Result<(Vec<u8>, ProtocolName), RequestFailure>, oneshot::Canceled>;
		fn block_response_into_blocks(
			&self,
			request: &BlockRequest<Block>,
			response: Vec<u8>,
		) -> Result<Vec<BlockData<Block>>, BlockResponseError>;
	}
}
