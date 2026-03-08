// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tagged Transaction Queue Runtime API.

use crate::runtime::{
	traits::Block as BlockT,
	transaction_validity::{TransactionSource, TransactionValidity},
};

crate::api::decl_runtime_apis! {
	/// The `TaggedTransactionQueue` api trait for interfering with the transaction queue.
	#[api_version(3)]
	pub trait TaggedTransactionQueue {
		/// Validate the transaction.
		#[changed_in(2)]
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity;

		/// Validate the transaction.
		#[changed_in(3)]
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity;

		/// Validate the transaction.
		///
		/// This method is invoked by the transaction pool to learn details about given transaction.
		/// The implementation should make sure to verify the correctness of the transaction
		/// against current state. The given `block_hash` corresponds to the hash of the block
		/// that is used as current state.
		///
		/// Note that this call may be performed by the pool multiple times and transactions
		/// might be verified in any possible order.
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: Block::Hash,
		) -> TransactionValidity;
	}
}
