// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Common components re-used across different txpool implementations.

pub(crate) mod api;
pub(crate) mod enactment_state;
pub(crate) mod error;
pub(crate) mod metrics;
pub(crate) mod sliding_stat;
#[cfg(test)]
pub(crate) mod tests;
pub(crate) mod tracing_log_xt;

use futures::StreamExt;
use std::sync::Arc;

/// Stat sliding window, in seconds for per-transaction activities.
pub(crate) const STAT_SLIDING_WINDOW: u64 = 3;

/// Inform the transaction pool about imported and finalized blocks.
pub async fn notification_future<Client, Pool, Block>(client: Arc<Client>, txpool: Arc<Pool>)
where
	Block: subsoil::runtime::traits::Block,
	Client: soil_client::client_api::BlockchainEvents<Block>,
	Pool: soil_client::transaction_pool::MaintainedTransactionPool<Block = Block>,
{
	let import_stream = client
		.import_notification_stream()
		.filter_map(|n| futures::future::ready(n.try_into().ok()))
		.fuse();
	let finality_stream = client.finality_notification_stream().map(Into::into).fuse();

	futures::stream::select(import_stream, finality_stream)
		.for_each(|evt| txpool.maintain(evt))
		.await
}
