// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Error types for consensus modules.

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for consensus-related operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Missing state at block with given descriptor.
	#[error("State unavailable at block {0}")]
	StateUnavailable(String),
	/// Intermediate missing.
	#[error("Missing intermediate")]
	NoIntermediate,
	/// Intermediate is of wrong type.
	#[error("Invalid intermediate")]
	InvalidIntermediate,
	/// Error checking signature.
	#[error("Message signature {0:?} by {1:?} is invalid")]
	InvalidSignature(Vec<u8>, Vec<u8>),
	/// Invalid authorities set received from the runtime.
	#[error("Current state of blockchain has invalid authorities set")]
	InvalidAuthoritiesSet,
	/// Justification requirements not met.
	#[error("Invalid justification")]
	InvalidJustification,
	/// The justification provided is outdated and corresponds to a previous set.
	#[error("Import failed with outdated justification")]
	OutdatedJustification,
	/// Error from the client while importing.
	#[error("Import failed: {0}")]
	ClientImport(String),
	/// Error from the client while fetching some data from the chain.
	#[error("Chain lookup failed: {0}")]
	ChainLookup(String),
	/// Signing failed.
	#[error("Failed to sign: {0}")]
	CannotSign(String),
	/// Some other error.
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
}
