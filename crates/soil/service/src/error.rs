// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Errors that can occur during the service operation.

use soil_client::keystore;
use soil_client::blockchain;
use soil_client::consensus;

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
	Prometheus(#[from] prometheus_endpoint::PrometheusError),

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
