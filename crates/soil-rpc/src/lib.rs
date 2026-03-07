// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Substrate RPC primitives and utilities.

#![warn(missing_docs)]

pub mod api;
pub mod author;
pub mod chain;
pub mod dev;
pub mod list;
pub mod mixnet;
pub mod mmr;
pub mod number;
pub mod offchain;
pub mod server;
pub mod state;
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
