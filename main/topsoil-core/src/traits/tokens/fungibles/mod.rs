// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The traits for *sets* of [`fungible`](`topsoil_core::traits::fungible`) tokens and any
//! associated types.
//!
//! Individual tokens in the `fungibles` set may be used when a `fungible` trait is expected using
//! [`crate::traits::tokens::fungible::ItemOf`].
//!
//! Also see the [`frame_tokens`] reference docs for more information about the place of
//! `fungible` traits in Substrate.
//!
//! [`frame_tokens`]: ../../../../polkadot_sdk_docs/reference_docs/frame_tokens/index.html

pub mod approvals;
mod enumerable;
pub mod freeze;
pub mod hold;
pub(crate) mod imbalance;
mod lifetime;
pub mod metadata;
mod regular;
pub mod roles;
mod union_of;

pub use enumerable::Inspect as InspectEnumerable;
pub use freeze::{Inspect as InspectFreeze, Mutate as MutateFreeze};
pub use hold::{
	Balanced as BalancedHold, Inspect as InspectHold, Mutate as MutateHold,
	Unbalanced as UnbalancedHold,
};
pub use imbalance::{Credit, Debt, HandleImbalanceDrop, Imbalance};
pub use lifetime::{Create, Destroy, Refund};
pub use regular::{
	Balanced, DecreaseIssuance, Dust, IncreaseIssuance, Inspect, Mutate, Unbalanced,
};
pub use union_of::UnionOf;
