// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Errors types of runtime utilities.

/// Generic result for the runtime utilities.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the runtime utilities.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
	#[error("Scale codec error: {0}")]
	ScaleCodec(#[from] codec::Error),
	#[error("Opaque metadata not found")]
	OpaqueMetadataNotFound,
	#[error("Stable metadata version not found")]
	StableMetadataVersionNotFound,
	#[error("WASM executor error: {0}")]
	Executor(#[from] soil_client::executor::common::error::Error),
}
