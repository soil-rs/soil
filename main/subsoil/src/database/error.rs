// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// The error type for database operations.
#[derive(Debug)]
pub struct DatabaseError(pub Box<dyn std::error::Error + Send + Sync + 'static>);

impl std::fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl std::error::Error for DatabaseError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

/// A specialized `Result` type for database operations.
pub type Result<T> = std::result::Result<T, DatabaseError>;
