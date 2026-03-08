// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Shim for litep2p's Bitswap implementation to make it work with `soil-network`.

use crate::bitswap::is_cid_supported;
use futures::StreamExt;
use litep2p::protocol::libp2p::bitswap::{
	BitswapEvent, BitswapHandle, BlockPresenceType, Config, ResponseType, WantType,
};

use soil_client::client_api::BlockBackend;
use subsoil::runtime::traits::Block as BlockT;

use std::{future::Future, pin::Pin, sync::Arc};

/// Logging target for the file.
const LOG_TARGET: &str = "sub-libp2p::bitswap";

pub struct BitswapServer<Block: BlockT> {
	/// Bitswap handle.
	handle: BitswapHandle,

	/// Blockchain client.
	client: Arc<dyn BlockBackend<Block> + Send + Sync>,
}

impl<Block: BlockT> BitswapServer<Block> {
	/// Create new [`BitswapServer`].
	pub fn new(
		client: Arc<dyn BlockBackend<Block> + Send + Sync>,
	) -> (Pin<Box<dyn Future<Output = ()> + Send>>, Config) {
		let (config, handle) = Config::new();
		let bitswap = Self { client, handle };

		(Box::pin(async move { bitswap.run().await }), config)
	}

	async fn run(mut self) {
		log::debug!(target: LOG_TARGET, "starting bitswap server");

		while let Some(event) = self.handle.next().await {
			match event {
				BitswapEvent::Request { peer, cids } => {
					log::debug!(target: LOG_TARGET, "handle bitswap request from {peer:?} for {cids:?}");

					let response: Vec<ResponseType> = cids
						.into_iter()
						.filter(|(cid, _)| is_cid_supported(&cid))
						.map(|(cid, want_type)| {
							let mut hash = Block::Hash::default();
							hash.as_mut().copy_from_slice(&cid.hash().digest()[0..32]);
							let transaction = match self.client.indexed_transaction(hash) {
								Ok(ex) => ex,
								Err(error) => {
									log::error!(target: LOG_TARGET, "error retrieving transaction {hash}: {error}");
									None
								},
							};

							match transaction {
								Some(transaction) => {
									log::trace!(target: LOG_TARGET, "found cid {cid:?}, hash {hash:?}");

									match want_type {
										WantType::Block => {
											ResponseType::Block { cid, block: transaction }
										},
										_ => ResponseType::Presence {
											cid,
											presence: BlockPresenceType::Have,
										},
									}
								},
								None => {
									log::trace!(target: LOG_TARGET, "missing cid {cid:?}, hash {hash:?}");

									ResponseType::Presence {
										cid,
										presence: BlockPresenceType::DontHave,
									}
								},
							}
						})
						.collect();

					self.handle.send_response(peer, response).await;
				},
				BitswapEvent::Response { peer, responses } => {
					// We're a server, not a client - ignore incoming responses
					log::trace!(
						target: LOG_TARGET,
						"ignoring bitswap response from {peer:?} with {} entries",
						responses.len()
					);
				},
			}
		}
	}
}
