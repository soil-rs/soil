// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("IO Error")]
	IoError(#[from] std::io::Error),
	#[error("This telemetry instance has already been initialized!")]
	TelemetryAlreadyInitialized,
	#[error("The telemetry worker has been dropped already.")]
	TelemetryWorkerDropped,
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;
