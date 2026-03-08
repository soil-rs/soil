// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Configuration of the transaction protocol

use futures::prelude::*;
use soil_network::common::ExHashT;
use soil_network::MAX_RESPONSE_SIZE;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc, time};
use subsoil::runtime::traits::Block as BlockT;

/// Interval at which we propagate transactions;
pub(crate) const PROPAGATE_TIMEOUT: time::Duration = time::Duration::from_millis(2900);

/// Maximum number of known transaction hashes to keep for a peer.
///
/// This should be approx. 2 blocks full of transactions for the network to function properly.
pub(crate) const MAX_KNOWN_TRANSACTIONS: usize = 10240; // ~300kb per peer + overhead.

/// Maximum allowed size for a transactions notification.
pub(crate) const MAX_TRANSACTIONS_SIZE: u64 = MAX_RESPONSE_SIZE;

/// Maximum number of transaction validation request we keep at any moment.
pub(crate) const MAX_PENDING_TRANSACTIONS: usize = 8192;

/// Result of the transaction import.
#[derive(Clone, Copy, Debug)]
pub enum TransactionImport {
	/// Transaction is good but already known by the transaction pool.
	KnownGood,
	/// Transaction is good and not yet known.
	NewGood,
	/// Transaction is invalid.
	Bad,
	/// Transaction import was not performed.
	None,
}

/// Future resolving to transaction import result.
pub type TransactionImportFuture = Pin<Box<dyn Future<Output = TransactionImport> + Send>>;

/// Transaction pool interface
pub trait TransactionPool<H: ExHashT, B: BlockT>: Send + Sync {
	/// Get transactions from the pool that are ready to be propagated.
	fn transactions(&self) -> Vec<(H, Arc<B::Extrinsic>)>;
	/// Get hash of transaction.
	fn hash_of(&self, transaction: &B::Extrinsic) -> H;
	/// Import a transaction into the pool.
	///
	/// This will return future.
	fn import(&self, transaction: B::Extrinsic) -> TransactionImportFuture;
	/// Notify the pool about transactions broadcast.
	fn on_broadcasted(&self, propagations: HashMap<H, Vec<String>>);
	/// Get transaction by hash.
	fn transaction(&self, hash: &H) -> Option<Arc<B::Extrinsic>>;
}

/// Dummy implementation of the [`TransactionPool`] trait for a transaction pool that is always
/// empty and discards all incoming transactions.
///
/// Requires the "hash" type to implement the `Default` trait.
///
/// Useful for testing purposes.
pub struct EmptyTransactionPool;

impl<H: ExHashT + Default, B: BlockT> TransactionPool<H, B> for EmptyTransactionPool {
	fn transactions(&self) -> Vec<(H, Arc<B::Extrinsic>)> {
		Vec::new()
	}

	fn hash_of(&self, _transaction: &B::Extrinsic) -> H {
		Default::default()
	}

	fn import(&self, _transaction: B::Extrinsic) -> TransactionImportFuture {
		Box::pin(future::ready(TransactionImport::KnownGood))
	}

	fn on_broadcasted(&self, _: HashMap<H, Vec<String>>) {}

	fn transaction(&self, _h: &H) -> Option<Arc<B::Extrinsic>> {
		None
	}
}
