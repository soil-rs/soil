// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use alloc::vec::Vec;
use codec::{Decode, Encode};
pub use crate::core::crypto::KeyTypeId;
use crate::runtime::traits::GeneratedSessionKeys;

/// Opaque [`GeneratedSessionKeys`].
#[derive(Debug, Default, Decode, Encode, scale_info::TypeInfo)]
pub struct OpaqueGeneratedSessionKeys {
	/// The public session keys.
	pub keys: Vec<u8>,
	/// The proof proving the ownership of the public session keys for some owner.
	pub proof: Vec<u8>,
}

impl<K: Encode, P: Encode> From<GeneratedSessionKeys<K, P>> for OpaqueGeneratedSessionKeys {
	fn from(value: GeneratedSessionKeys<K, P>) -> Self {
		Self { keys: value.keys.encode(), proof: value.proof.encode() }
	}
}

crate::api::decl_runtime_apis! {
	/// Session keys runtime api.
	#[api_version(2)]
	pub trait SessionKeys {
		/// Generate a set of session keys with optionally using the given seed.
		/// The keys should be stored within the keystore exposed via runtime
		/// externalities.
		///
		/// The seed needs to be a valid `utf8` string.
		///
		/// Returns the concatenated SCALE encoded public keys.
		fn generate_session_keys(owner: Vec<u8>, seed: Option<Vec<u8>>) -> OpaqueGeneratedSessionKeys;

		#[changed_in(2)]
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8>;

		/// Decode the given public session keys.
		///
		/// Returns the list of public raw public keys + key type.
		fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>>;
	}
}
