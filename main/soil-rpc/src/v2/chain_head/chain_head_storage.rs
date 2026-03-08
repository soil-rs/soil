// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Implementation of the `chainHead_storage` method.

use std::{marker::PhantomData, sync::Arc};

use soil_client::client_api::{Backend, ChildInfo, StorageKey, StorageProvider};
use subsoil::runtime::traits::Block as BlockT;
use tokio::sync::mpsc;

use crate::v2::common::{
	events::{StorageQuery, StorageQueryType},
	storage::{IterQueryType, QueryIter, QueryResult, Storage},
};

/// Generates the events of the `chainHead_storage` method.
pub struct ChainHeadStorage<Client, Block, BE> {
	/// Storage client.
	client: Storage<Client, Block, BE>,
	_phandom: PhantomData<(BE, Block)>,
}

impl<Client, Block, BE> Clone for ChainHeadStorage<Client, Block, BE> {
	fn clone(&self) -> Self {
		Self { client: self.client.clone(), _phandom: PhantomData }
	}
}

impl<Client, Block, BE> ChainHeadStorage<Client, Block, BE> {
	/// Constructs a new [`ChainHeadStorage`].
	pub fn new(client: Arc<Client>) -> Self {
		Self { client: Storage::new(client), _phandom: PhantomData }
	}
}

impl<Client, Block, BE> ChainHeadStorage<Client, Block, BE>
where
	Block: BlockT + 'static,
	BE: Backend<Block> + 'static,
	Client: StorageProvider<Block, BE> + Send + Sync + 'static,
{
	/// Generate the block events for the `chainHead_storage` method.
	pub async fn generate_events(
		&mut self,
		hash: Block::Hash,
		items: Vec<StorageQuery<StorageKey>>,
		child_key: Option<ChildInfo>,
		tx: mpsc::Sender<QueryResult>,
	) -> Result<(), tokio::task::JoinError> {
		let this = self.clone();

		tokio::task::spawn_blocking(move || {
			for item in items {
				match item.query_type {
					StorageQueryType::Value => {
						let rp = this.client.query_value(hash, &item.key, child_key.as_ref());
						if tx.blocking_send(rp).is_err() {
							break;
						}
					},
					StorageQueryType::Hash => {
						let rp = this.client.query_hash(hash, &item.key, child_key.as_ref());
						if tx.blocking_send(rp).is_err() {
							break;
						}
					},
					StorageQueryType::ClosestDescendantMerkleValue => {
						let rp =
							this.client.query_merkle_value(hash, &item.key, child_key.as_ref());
						if tx.blocking_send(rp).is_err() {
							break;
						}
					},
					StorageQueryType::DescendantsValues => {
						let query = QueryIter {
							query_key: item.key,
							ty: IterQueryType::Value,
							pagination_start_key: None,
						};
						this.client.query_iter_pagination_with_producer(
							query,
							hash,
							child_key.as_ref(),
							&tx,
						)
					},
					StorageQueryType::DescendantsHashes => {
						let query = QueryIter {
							query_key: item.key,
							ty: IterQueryType::Hash,
							pagination_start_key: None,
						};
						this.client.query_iter_pagination_with_producer(
							query,
							hash,
							child_key.as_ref(),
							&tx,
						)
					},
				}
			}
		})
		.await?;

		Ok(())
	}
}
