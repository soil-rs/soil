// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Authority discovery pallet and primitives.
//!
//! This crate provides:
//! - Primitive types (`AuthorityId`, `AuthorityPair`, `AuthoritySignature`) and the
//!   `AuthorityDiscoveryApi` runtime API.
//! - A FRAME pallet for tracking current and next authority sets.
//! - A client-side service (behind the `std` feature) for publishing and discovering
//!   authority addresses on the DHT.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

mod app {
	use subsoil::application_crypto::{key_types::AUTHORITY_DISCOVERY, sr25519};
	subsoil::app_crypto!(sr25519, AUTHORITY_DISCOVERY);
}

subsoil::with_pair! {
	/// An authority discovery authority keypair.
	pub type AuthorityPair = app::Pair;
}

/// An authority discovery authority identifier.
pub type AuthorityId = app::Public;

/// An authority discovery authority signature.
pub type AuthoritySignature = app::Signature;

subsoil::api::decl_runtime_apis! {
	/// The authority discovery api.
	///
	/// This api is used by the `client/authority-discovery` module to retrieve identifiers
	/// of the current and next authority set.
	pub trait AuthorityDiscoveryApi {
		/// Retrieve authority identifiers of the current and next authority set.
		fn authorities() -> Vec<AuthorityId>;
	}
}

pub mod pallet;
pub use pallet::*;

#[cfg(feature = "std")]
pub mod client;
