// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Configuration of the statement protocol

use std::time;

/// Interval at which we propagate statements;
pub(crate) const PROPAGATE_TIMEOUT: time::Duration = time::Duration::from_millis(1000);

/// Maximum number of known statement hashes to keep for a peer.
pub const MAX_KNOWN_STATEMENTS: usize = 4 * 1024 * 1024; // * 32 bytes for hash = 128 MB per peer

/// Maximum allowed size for a statement notification.
pub const MAX_STATEMENT_NOTIFICATION_SIZE: u64 = 1024 * 1024;

/// Maximum number of statement validation request we keep at any moment.
pub const MAX_PENDING_STATEMENTS: usize = 2 * 1024 * 1024;

/// Default maximum statements per second before rate limiting kicks in.
pub const DEFAULT_STATEMENTS_PER_SECOND: u32 = 50_000;

/// Burst capacity coefficient for the rate limiter.
pub const STATEMENTS_BURST_COEFFICIENT: u32 = 5;
