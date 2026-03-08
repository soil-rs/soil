// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Shared test primitives.

use codec::{Decode, DecodeWithMemTracking, Encode};

pub use subsoil::application_crypto as soil_application_crypto;
use subsoil::application_crypto::sr25519;

use std::vec::Vec;
pub use subsoil::core::hash::H256;
use subsoil::runtime::{
	traits::{BlakeTwo256, ExtrinsicLike, LazyExtrinsic, Verify},
	OpaqueExtrinsic,
};

/// Extrinsic for test-runtime.
#[derive(
	Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, Debug, scale_info::TypeInfo,
)]
pub enum Extrinsic {
	IncludeData(Vec<u8>),
	StorageChange(Vec<u8>, Option<Vec<u8>>),
}

impl From<Extrinsic> for OpaqueExtrinsic {
	fn from(xt: Extrinsic) -> Self {
		OpaqueExtrinsic::from_blob(xt.encode())
	}
}

impl LazyExtrinsic for Extrinsic {
	fn decode_unprefixed(data: &[u8]) -> Result<Self, codec::Error> {
		Self::decode(&mut &data[..])
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for Extrinsic {
	fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error>
	where
		S: ::serde::Serializer,
	{
		self.using_encoded(|bytes| seq.serialize_bytes(bytes))
	}
}

impl ExtrinsicLike for Extrinsic {
	fn is_signed(&self) -> Option<bool> {
		if let Extrinsic::IncludeData(_) = *self {
			Some(false)
		} else {
			Some(true)
		}
	}

	fn is_bare(&self) -> bool {
		if let Extrinsic::IncludeData(_) = *self {
			true
		} else {
			false
		}
	}
}

/// The signature type used by accounts/transactions.
pub type AccountSignature = sr25519::Signature;
/// An identifier for an account on this system.
pub type AccountId = <AccountSignature as Verify>::Signer;
/// A simple hash type for all our hashing.
pub type Hash = H256;
/// The block number type used in this runtime.
pub type BlockNumber = u64;
/// Index of a transaction.
pub type Nonce = u64;
/// The item of a block digest.
pub type DigestItem = subsoil::runtime::generic::DigestItem;
/// The digest of a block.
pub type Digest = subsoil::runtime::generic::Digest;
/// A test block.
pub type Block = subsoil::runtime::generic::Block<Header, Extrinsic>;
/// A test block's header.
pub type Header = subsoil::runtime::generic::Header<BlockNumber, BlakeTwo256>;
