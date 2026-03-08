// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate transaction pool implementation.

#![recursion_limit = "256"]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod builder;
mod common;
mod fork_aware_txpool;
mod graph;
mod single_state_txpool;
mod transaction_pool_wrapper;

use common::{api, enactment_state};
use std::sync::Arc;

pub use api::FullChainApi;
pub use builder::{Builder, TransactionPoolHandle, TransactionPoolOptions, TransactionPoolType};
pub use common::notification_future;
pub use fork_aware_txpool::{ForkAwareTxPool, ForkAwareTxPoolTask};
pub use graph::{
	base_pool::{Limit as PoolLimit, TimedTransactionSource},
	ChainApi, Options, Pool, ValidateTransactionPriority,
};
use single_state_txpool::prune_known_txs_for_block;
pub use single_state_txpool::{BasicPool, RevalidationType};
pub use transaction_pool_wrapper::TransactionPoolWrapper;

type BoxedReadyIterator<Hash, Data> = Box<
	dyn soil_client::transaction_pool::ReadyTransactions<
			Item = Arc<graph::base_pool::Transaction<Hash, Data>>,
		> + Send,
>;

type ReadyIteratorFor<PoolApi> =
	BoxedReadyIterator<graph::ExtrinsicHash<PoolApi>, graph::ExtrinsicFor<PoolApi>>;

/// Log target for transaction pool.
///
/// It can be used by other components for logging functionality strictly related to txpool (e.g.
/// importing transaction).
pub const LOG_TARGET: &str = "txpool";
const LOG_TARGET_STAT: &str = "txpoolstats";
