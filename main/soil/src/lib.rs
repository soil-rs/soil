// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Umbrella crate for Soil client and service crates.
//!
//! This crate provides short module-style re-exports for the main `soil-*`
//! crates used to compose a node.
//!
//! The consensus algorithm crates are feature-gated. Enable them individually
//! via `aura`, `babe`, `beefy`, `grandpa`, `manual-seal`, and `pow`, or enable
//! all of them with `full`.
//!
//! # Examples
//!
//! ```
//! use soil::{client, consensus, service};
//! ```
//!
//! ```
//! # #[cfg(feature = "grandpa")]
//! use soil::grandpa;
//! ```

#[doc(inline)]
pub use soil_chain_spec as chain_spec;
#[doc(inline)]
pub use soil_cli as cli;
#[doc(inline)]
pub use soil_client as client;
#[doc(inline)]
pub use soil_consensus as consensus;
#[doc(inline)]
pub use soil_network as network;
#[doc(inline)]
pub use soil_offchain as offchain;
#[doc(inline)]
pub use soil_rpc as rpc;
#[doc(inline)]
pub use soil_service as service;
#[doc(inline)]
pub use soil_sync_state_rpc as sync_state_rpc;
#[doc(inline)]
pub use soil_telemetry as telemetry;
#[doc(inline)]
pub use soil_txpool as txpool;

#[cfg(feature = "aura")]
#[doc(inline)]
pub use soil_aura as aura;

#[cfg(feature = "babe")]
#[doc(inline)]
pub use soil_babe as babe;

#[cfg(feature = "beefy")]
#[doc(inline)]
pub use soil_beefy as beefy;

#[cfg(feature = "grandpa")]
#[doc(inline)]
pub use soil_grandpa as grandpa;

#[cfg(feature = "manual-seal")]
#[doc(inline)]
pub use soil_manual_seal as manual_seal;

#[cfg(feature = "pow")]
#[doc(inline)]
pub use soil_pow as pow;
