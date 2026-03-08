// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Support code for the runtime. A set of test accounts.

extern crate alloc;
use alloc::fmt;

/// Test account crypto for sr25519.
pub mod sr25519;

/// Test account crypto for ed25519.
pub mod ed25519;

/// Test account crypto for bandersnatch.
#[cfg(feature = "bandersnatch-experimental")]
pub mod bandersnatch;

#[cfg(feature = "bandersnatch-experimental")]
pub use bandersnatch::Keyring as BandersnatchKeyring;
pub use ed25519::Keyring as Ed25519Keyring;
pub use sr25519::Keyring as Sr25519Keyring;

#[derive(Debug)]
/// Represents an error that occurs when parsing a string into a `KeyRing`.
pub struct ParseKeyringError;

impl fmt::Display for ParseKeyringError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ParseKeyringError")
	}
}
