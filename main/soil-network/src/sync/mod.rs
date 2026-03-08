// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Blockchain syncing implementation in Substrate.

pub use schema::v1::*;
pub use service::syncing_service::SyncingService;
pub use strategy::warp::{WarpSyncConfig, WarpSyncPhase, WarpSyncProgress};
pub use types::{SyncEvent, SyncEventStream, SyncState, SyncStatus, SyncStatusProvider};

mod block_announce_validator;
mod futures_stream;
mod justification_requests;
mod pending_responses;
mod schema;
pub mod types;

pub mod block_relay_protocol;
pub mod block_request_handler;
pub mod blocks;
pub mod engine;
pub mod mock;
pub mod service;
pub mod state_request_handler;
pub mod strategy;
pub mod warp_request_handler;

/// Log target for this crate.
const LOG_TARGET: &str = "sync";
