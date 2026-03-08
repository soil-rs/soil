// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Errors that can occur during the service operation.

use soil_client::blockchain;
use soil_client::consensus;
use soil_client::keystore;

/// Service Result typedef.
pub type Result<T> = std::result::Result<T, Error>;

/// Service errors.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Error {
	#[error(transparent)]
	Client(#[from] soil_client::blockchain::Error),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Consensus(#[from] soil_client::consensus::Error),

	#[error(transparent)]
	Network(#[from] soil_network::error::Error),

	#[error(transparent)]
	Keystore(#[from] soil_client::keystore::Error),

	#[error(transparent)]
	Telemetry(#[from] soil_telemetry::Error),

	#[error("Best chain selection strategy (SelectChain) is not provided.")]
	SelectChainRequired,

	#[error("Tasks executor hasn't been provided.")]
	TaskExecutorRequired,

	#[error("Prometheus metrics error: {0}")]
	Prometheus(#[from] soil_prometheus::PrometheusError),

	#[error("Application: {0}")]
	Application(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),

	#[error("Other: {0}")]
	Other(String),
}

impl<'a> From<&'a str> for Error {
	fn from(s: &'a str) -> Self {
		Error::Other(s.into())
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		Error::Other(s)
	}
}
