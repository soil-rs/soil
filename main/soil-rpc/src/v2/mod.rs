// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate JSON-RPC interface v2.
//!
//! Specification [document](https://paritytech.github.io/json-rpc-interface-spec/).

#![warn(missing_docs)]

use subsoil::core::hexdisplay::{AsBytesRef, HexDisplay};

mod common;

pub mod archive;
pub mod chain_head;
pub mod chain_spec;
pub mod transaction;

/// Task executor that is being used by RPC subscriptions.
pub type SubscriptionTaskExecutor = std::sync::Arc<dyn subsoil::core::traits::SpawnNamed>;

/// Util function to encode a value as a hex string
pub fn hex_string<Data: AsBytesRef>(data: &Data) -> String {
	format!("0x{:?}", HexDisplay::from(data))
}
