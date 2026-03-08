// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Transaction pool error.

use soil_client::transaction_pool::error::Error as TxPoolError;

/// Transaction pool result.
pub type Result<T> = std::result::Result<T, Error>;

/// Transaction pool error type.
#[derive(Debug, thiserror::Error, strum::AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
pub enum Error {
	#[error("Transaction pool error: {0}")]
	Pool(#[from] TxPoolError),

	#[error("Blockchain error: {0}")]
	Blockchain(#[from] soil_client::blockchain::Error),

	#[error("Block conversion error: {0}")]
	BlockIdConversion(String),

	#[error("Runtime error: {0}")]
	RuntimeApi(String),
}

impl soil_client::transaction_pool::error::IntoPoolError for Error {
	fn into_pool_error(self) -> std::result::Result<TxPoolError, Self> {
		match self {
			Error::Pool(e) => Ok(e),
			e => Err(e),
		}
	}
}

impl soil_client::transaction_pool::error::IntoMetricsLabel for Error {
	fn label(&self) -> String {
		self.as_ref().to_string()
	}
}
