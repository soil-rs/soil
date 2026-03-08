// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate RPC primitives and utilities.

#![warn(missing_docs)]

pub mod api;
pub mod author;
pub mod chain;
pub mod client;
pub mod dev;
pub mod list;
pub mod mixnet;
pub mod mmr;
pub mod number;
pub mod offchain;
pub mod server;
pub mod state;
pub mod state_trie_migration;
pub mod statement;
pub mod system;
pub mod tracing;
pub mod utils;
pub mod v2;

pub use api::{check_if_safe, DenyUnsafe, UnsafeRpcError};
pub use jsonrpsee::core::id_providers::{
	RandomIntegerIdProvider as RandomIntegerSubscriptionId,
	RandomStringIdProvider as RandomStringSubscriptionId,
};

#[cfg(any(test, feature = "test-helpers"))]
pub mod testing;

/// Task executor that is being used by RPC subscriptions.
pub type SubscriptionTaskExecutor = std::sync::Arc<dyn subsoil::core::traits::SpawnNamed>;

/// A util function to assert the result of serialization and deserialization is the same.
#[cfg(test)]
pub(crate) fn assert_deser<T>(s: &str, expected: T)
where
	T: std::fmt::Debug + serde::ser::Serialize + serde::de::DeserializeOwned + PartialEq,
{
	assert_eq!(serde_json::from_str::<T>(s).unwrap(), expected);
	assert_eq!(serde_json::to_string(&expected).unwrap(), s);
}
