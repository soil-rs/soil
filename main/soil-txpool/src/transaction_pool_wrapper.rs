// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Transaction pool wrapper. Provides a type for wrapping object providing actual implementation of
//! transaction pool.

use crate::{
	builder::FullClientTransactionPool,
	graph::{base_pool::Transaction, ExtrinsicFor, ExtrinsicHash},
	ChainApi, FullChainApi, ReadyIteratorFor,
};
use async_trait::async_trait;
use soil_client::transaction_pool::{
	ChainEvent, ImportNotificationStream, LocalTransactionFor, LocalTransactionPool,
	MaintainedTransactionPool, PoolStatus, ReadyTransactions, TransactionFor, TransactionPool,
	TransactionSource, TransactionStatusStreamFor, TxHash, TxInvalidityReportMap,
};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use subsoil::runtime::traits::Block as BlockT;

/// The wrapper for actual object providing implementation of TransactionPool.
///
/// This wraps actual implementation of the TransactionPool, e.g. fork-aware or single-state.
pub struct TransactionPoolWrapper<Block, Client>(
	pub Box<dyn FullClientTransactionPool<Block, Client>>,
)
where
	Block: BlockT,
	Client: subsoil::api::ProvideRuntimeApi<Block>
		+ soil_client::client_api::BlockBackend<Block>
		+ soil_client::client_api::blockchain::HeaderBackend<Block>
		+ subsoil::runtime::traits::BlockIdTo<Block>
		+ soil_client::blockchain::HeaderMetadata<Block, Error = soil_client::blockchain::Error>
		+ 'static,
	Client::Api: subsoil::txpool::runtime_api::TaggedTransactionQueue<Block>;

#[async_trait]
impl<Block, Client> TransactionPool for TransactionPoolWrapper<Block, Client>
where
	Block: BlockT,
	Client: subsoil::api::ProvideRuntimeApi<Block>
		+ soil_client::client_api::BlockBackend<Block>
		+ soil_client::client_api::blockchain::HeaderBackend<Block>
		+ subsoil::runtime::traits::BlockIdTo<Block>
		+ soil_client::blockchain::HeaderMetadata<Block, Error = soil_client::blockchain::Error>
		+ 'static,
	Client::Api: subsoil::txpool::runtime_api::TaggedTransactionQueue<Block>,
{
	type Block = Block;
	type Hash = ExtrinsicHash<FullChainApi<Client, Block>>;
	type InPoolTransaction = Transaction<
		ExtrinsicHash<FullChainApi<Client, Block>>,
		ExtrinsicFor<FullChainApi<Client, Block>>,
	>;
	type Error = <FullChainApi<Client, Block> as ChainApi>::Error;

	async fn submit_at(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xts: Vec<TransactionFor<Self>>,
	) -> Result<Vec<Result<TxHash<Self>, Self::Error>>, Self::Error> {
		self.0.submit_at(at, source, xts).await
	}

	async fn submit_one(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> Result<TxHash<Self>, Self::Error> {
		self.0.submit_one(at, source, xt).await
	}

	async fn submit_and_watch(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> Result<Pin<Box<TransactionStatusStreamFor<Self>>>, Self::Error> {
		self.0.submit_and_watch(at, source, xt).await
	}

	async fn ready_at(
		&self,
		at: <Self::Block as BlockT>::Hash,
	) -> ReadyIteratorFor<FullChainApi<Client, Block>> {
		self.0.ready_at(at).await
	}

	fn ready(&self) -> Box<dyn ReadyTransactions<Item = Arc<Self::InPoolTransaction>> + Send> {
		self.0.ready()
	}

	async fn report_invalid(
		&self,
		at: Option<<Self::Block as BlockT>::Hash>,
		invalid_tx_errors: TxInvalidityReportMap<TxHash<Self>>,
	) -> Vec<Arc<Self::InPoolTransaction>> {
		self.0.report_invalid(at, invalid_tx_errors).await
	}

	fn futures(&self) -> Vec<Self::InPoolTransaction> {
		self.0.futures()
	}

	fn status(&self) -> PoolStatus {
		self.0.status()
	}

	fn import_notification_stream(&self) -> ImportNotificationStream<TxHash<Self>> {
		self.0.import_notification_stream()
	}

	fn on_broadcasted(&self, propagations: HashMap<TxHash<Self>, Vec<String>>) {
		self.0.on_broadcasted(propagations)
	}

	fn hash_of(&self, xt: &TransactionFor<Self>) -> TxHash<Self> {
		self.0.hash_of(xt)
	}

	fn ready_transaction(&self, hash: &TxHash<Self>) -> Option<Arc<Self::InPoolTransaction>> {
		self.0.ready_transaction(hash)
	}

	async fn ready_at_with_timeout(
		&self,
		at: <Self::Block as BlockT>::Hash,
		timeout: std::time::Duration,
	) -> ReadyIteratorFor<FullChainApi<Client, Block>> {
		self.0.ready_at_with_timeout(at, timeout).await
	}
}

#[async_trait]
impl<Block, Client> MaintainedTransactionPool for TransactionPoolWrapper<Block, Client>
where
	Block: BlockT,
	Client: subsoil::api::ProvideRuntimeApi<Block>
		+ soil_client::client_api::BlockBackend<Block>
		+ soil_client::client_api::blockchain::HeaderBackend<Block>
		+ subsoil::runtime::traits::BlockIdTo<Block>
		+ soil_client::blockchain::HeaderMetadata<Block, Error = soil_client::blockchain::Error>
		+ 'static,
	Client::Api: subsoil::txpool::runtime_api::TaggedTransactionQueue<Block>,
{
	async fn maintain(&self, event: ChainEvent<Self::Block>) {
		self.0.maintain(event).await;
	}
}

impl<Block, Client> LocalTransactionPool for TransactionPoolWrapper<Block, Client>
where
	Block: BlockT,
	Client: subsoil::api::ProvideRuntimeApi<Block>
		+ soil_client::client_api::BlockBackend<Block>
		+ soil_client::client_api::blockchain::HeaderBackend<Block>
		+ subsoil::runtime::traits::BlockIdTo<Block>
		+ soil_client::blockchain::HeaderMetadata<Block, Error = soil_client::blockchain::Error>
		+ 'static,
	Client::Api: subsoil::txpool::runtime_api::TaggedTransactionQueue<Block>,
{
	type Block = Block;
	type Hash = ExtrinsicHash<FullChainApi<Client, Block>>;
	type Error = <FullChainApi<Client, Block> as ChainApi>::Error;

	fn submit_local(
		&self,
		at: <Self::Block as BlockT>::Hash,
		xt: LocalTransactionFor<Self>,
	) -> Result<Self::Hash, Self::Error> {
		self.0.submit_local(at, xt)
	}
}
