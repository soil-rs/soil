// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use clap::{Args, ValueEnum};
use soil_txpool::TransactionPoolOptions;

/// Type of transaction pool to be used
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum TransactionPoolType {
	/// Uses a legacy, single-state transaction pool.
	SingleState,
	/// Uses a fork-aware transaction pool.
	ForkAware,
}

impl Into<soil_txpool::TransactionPoolType> for TransactionPoolType {
	fn into(self) -> soil_txpool::TransactionPoolType {
		match self {
			TransactionPoolType::SingleState => soil_txpool::TransactionPoolType::SingleState,
			TransactionPoolType::ForkAware => soil_txpool::TransactionPoolType::ForkAware,
		}
	}
}

/// Parameters used to create the pool configuration.
#[derive(Debug, Clone, Args)]
pub struct TransactionPoolParams {
	/// Maximum number of transactions in the transaction pool.
	#[arg(long, value_name = "COUNT", default_value_t = 8192)]
	pub pool_limit: usize,

	/// Maximum number of kilobytes of all transactions stored in the pool.
	#[arg(long, value_name = "COUNT", default_value_t = 20480)]
	pub pool_kbytes: usize,

	/// How long a transaction is banned for.
	///
	/// If it is considered invalid. Defaults to 1800s.
	#[arg(long, value_name = "SECONDS")]
	pub tx_ban_seconds: Option<u64>,

	/// The type of transaction pool to be instantiated.
	#[arg(long, value_enum, default_value_t = TransactionPoolType::ForkAware)]
	pub pool_type: TransactionPoolType,
}

impl TransactionPoolParams {
	/// Fill the given `PoolConfiguration` by looking at the cli parameters.
	pub fn transaction_pool(&self, is_dev: bool) -> TransactionPoolOptions {
		TransactionPoolOptions::new_with_params(
			self.pool_limit,
			self.pool_kbytes * 1024,
			self.tx_ban_seconds,
			self.pool_type.into(),
			is_dev,
		)
	}
}
