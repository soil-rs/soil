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

//! Blockchain syncing implementation in Substrate.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use schema::v1::*;
#[cfg(feature = "std")]
pub use service::syncing_service::SyncingService;
#[cfg(feature = "std")]
pub use strategy::warp::{WarpSyncConfig, WarpSyncPhase, WarpSyncProgress};
#[cfg(feature = "std")]
pub use types::{SyncEvent, SyncEventStream, SyncState, SyncStatus, SyncStatusProvider};

#[cfg(feature = "std")]
mod block_announce_validator;
#[cfg(feature = "std")]
mod futures_stream;
#[cfg(feature = "std")]
mod justification_requests;
#[cfg(feature = "std")]
mod pending_responses;
#[cfg(feature = "std")]
mod schema;
#[cfg(feature = "std")]
pub mod types;

#[cfg(feature = "std")]
pub mod block_relay_protocol;
#[cfg(feature = "std")]
pub mod block_request_handler;
#[cfg(feature = "std")]
pub mod blocks;
#[cfg(feature = "std")]
pub mod engine;
#[cfg(feature = "std")]
pub mod mock;
#[cfg(feature = "std")]
pub mod service;
#[cfg(feature = "std")]
pub mod state_request_handler;
#[cfg(feature = "std")]
pub mod strategy;
#[cfg(feature = "std")]
pub mod warp_request_handler;

/// Log target for this crate.
#[cfg(feature = "std")]
const LOG_TARGET: &str = "sync";
