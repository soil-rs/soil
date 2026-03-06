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

//! Traits and macros for constructing application specific strongly typed crypto wrappers.

pub use crate::core::crypto::{key_types, CryptoTypeId, DeriveJunction, KeyTypeId, Ss58Codec};
#[doc(hidden)]
pub use crate::core::crypto::{DeriveError, Pair, SecretStringError};
#[doc(hidden)]
pub use crate::core::{
	self as core_,
	crypto::{ByteArray, CryptoType, Derive, IsWrappedBy, Public, Signature, UncheckedFrom, Wraps},
	proof_of_possession::{ProofOfPossessionGenerator, ProofOfPossessionVerifier},
};

#[doc(hidden)]
pub use alloc::vec::Vec;
#[doc(hidden)]
pub use codec;
#[doc(hidden)]
pub use ::core::ops::Deref;
#[doc(hidden)]
pub use scale_info;
#[doc(hidden)]
#[cfg(feature = "serde")]
pub use serde;

#[cfg(feature = "bandersnatch-experimental")]
pub mod bandersnatch;
#[cfg(feature = "bls-experimental")]
pub mod bls381;
pub mod ecdsa;
#[cfg(feature = "bls-experimental")]
pub mod ecdsa_bls381;
pub mod ed25519;
pub mod sr25519;
mod traits;

pub use traits::*;

/// Declares `Public`, `Pair`, `Signature` and `ProofOfPossession` types which are functionally
/// equivalent to the corresponding types defined by `$module` but are new application-specific
/// types whose identifier is `$key_type`.
///
/// ```rust
/// # use subsoil::{app_crypto, application_crypto::{ed25519, KeyTypeId}};
/// // Declare a new set of crypto types using ed25519 logic that identifies as `KeyTypeId`
/// // of value `b"fuba"`.
/// app_crypto!(ed25519, KeyTypeId(*b"fuba"));
/// ```
#[cfg(feature = "full_crypto")]
#[macro_export]
macro_rules! app_crypto {
	($module:ident, $key_type:expr) => {
		$crate::app_crypto_public_full_crypto!($module::Public, $key_type, $module::CRYPTO_ID);
		$crate::app_crypto_public_common!(
			$module::Public,
			$module::Signature,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_signature_full_crypto!(
			$module::Signature,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_signature_common!($module::Signature, $key_type);
		$crate::app_crypto_proof_of_possession_full_crypto!(
			$module::ProofOfPossession,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_proof_of_possession_common!($module::ProofOfPossession, $key_type);
		$crate::app_crypto_pair_common!($module::Pair, $key_type, $module::CRYPTO_ID);
	};
}

/// Declares `Public`, `Pair` and `Signature` types which are functionally equivalent
/// to the corresponding types defined by `$module` but that are new application-specific
/// types whose identifier is `$key_type`.
///
/// ```rust
/// # use subsoil::{app_crypto, application_crypto::{ed25519, KeyTypeId}};
/// // Declare a new set of crypto types using ed25519 logic that identifies as `KeyTypeId`
/// // of value `b"fuba"`.
/// app_crypto!(ed25519, KeyTypeId(*b"fuba"));
/// ```
#[cfg(not(feature = "full_crypto"))]
#[macro_export]
macro_rules! app_crypto {
	($module:ident, $key_type:expr) => {
		$crate::app_crypto_public_not_full_crypto!($module::Public, $key_type, $module::CRYPTO_ID);
		$crate::app_crypto_public_common!(
			$module::Public,
			$module::Signature,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_signature_not_full_crypto!(
			$module::Signature,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_signature_common!($module::Signature, $key_type);
		$crate::app_crypto_proof_of_possession_not_full_crypto!(
			$module::ProofOfPossession,
			$key_type,
			$module::CRYPTO_ID
		);
		$crate::app_crypto_proof_of_possession_common!($module::ProofOfPossession, $key_type);
		$crate::app_crypto_pair_common!($module::Pair, $key_type, $module::CRYPTO_ID);
	};
}

/// Declares `Pair` type which is functionally equivalent to `$pair`, but is
/// new application-specific type whose identifier is `$key_type`.
/// It is a common part shared between full_crypto and non full_crypto environments.
#[macro_export]
macro_rules! app_crypto_pair_common {
	($pair:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $pair crypto; this has no specific App.
			#[derive(Clone)]
			pub struct Pair($pair);
		}

		impl $crate::application_crypto::CryptoType for Pair {
			type Pair = Pair;
		}

		impl $crate::application_crypto::Pair for Pair {
			type Public = Public;
			type Seed = <$pair as $crate::application_crypto::Pair>::Seed;
			type Signature = Signature;
			type ProofOfPossession = <$pair as $crate::application_crypto::Pair>::ProofOfPossession;

			$crate::app_crypto_pair_functions_if_std!($pair);
			$crate::app_crypto_pair_functions_if_full_crypto!($pair);

			fn from_phrase(
				phrase: &str,
				password: Option<&str>,
			) -> Result<(Self, Self::Seed), $crate::application_crypto::SecretStringError> {
				<$pair>::from_phrase(phrase, password).map(|r| (Self(r.0), r.1))
			}
			fn derive<Iter: Iterator<Item = $crate::application_crypto::DeriveJunction>>(
				&self,
				path: Iter,
				seed: Option<Self::Seed>,
			) -> Result<(Self, Option<Self::Seed>), $crate::application_crypto::DeriveError> {
				self.0.derive(path, seed).map(|x| (Self(x.0), x.1))
			}
			fn from_seed(seed: &Self::Seed) -> Self {
				Self(<$pair>::from_seed(seed))
			}
			fn from_seed_slice(seed: &[u8]) -> Result<Self, $crate::application_crypto::SecretStringError> {
				<$pair>::from_seed_slice(seed).map(Self)
			}
			fn verify<M: AsRef<[u8]>>(
				sig: &Self::Signature,
				message: M,
				pubkey: &Self::Public,
			) -> bool {
				<$pair>::verify(&sig.0, message, pubkey.as_ref())
			}
			fn public(&self) -> Self::Public {
				Public(self.0.public())
			}
			fn to_raw_vec(&self) -> $crate::application_crypto::Vec<u8> {
				self.0.to_raw_vec()
			}
		}

		impl $crate::application_crypto::ProofOfPossessionVerifier for Pair {
			fn verify_proof_of_possession(
				owner: &[u8],
				proof_of_possession: &Self::ProofOfPossession,
				allegedly_possessed_pubkey: &Self::Public,
			) -> bool {
				<$pair>::verify_proof_of_possession(
					owner,
					&proof_of_possession,
					allegedly_possessed_pubkey.as_ref(),
				)
			}
		}

		impl $crate::application_crypto::AppCrypto for Pair {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}

		impl $crate::application_crypto::AppPair for Pair {
			type Generic = $pair;
		}

		impl Pair {
			/// Convert into wrapped generic key pair type.
			pub fn into_inner(self) -> $pair {
				self.0
			}
		}
	};
}

/// Implements functions for the `Pair` trait when `feature = "std"` is enabled.
#[doc(hidden)]
#[cfg(feature = "std")]
#[macro_export]
macro_rules! app_crypto_pair_functions_if_std {
	($pair:ty) => {
		fn generate_with_phrase(password: Option<&str>) -> (Self, String, Self::Seed) {
			let r = <$pair>::generate_with_phrase(password);
			(Self(r.0), r.1, r.2)
		}
	};
}

#[doc(hidden)]
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! app_crypto_pair_functions_if_std {
	($pair:ty) => {};
}

/// Implements functions for the `Pair` trait when `feature = "full_crypto"` is enabled.
#[doc(hidden)]
#[cfg(feature = "full_crypto")]
#[macro_export]
macro_rules! app_crypto_pair_functions_if_full_crypto {
	($pair:ty) => {
		fn sign(&self, msg: &[u8]) -> Self::Signature {
			Signature(self.0.sign(msg))
		}
	};
}

#[doc(hidden)]
#[cfg(not(feature = "full_crypto"))]
#[macro_export]
macro_rules! app_crypto_pair_functions_if_full_crypto {
	($pair:ty) => {};
}

/// Declares `Public` type which is functionally equivalent to `$public` but is
/// new application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_public_common!` must be called too.
/// Can only be used with `full_crypto` feature.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_public_full_crypto {
	($public:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(
				Clone, Eq, Hash, PartialEq, PartialOrd, Ord,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::codec::MaxEncodedLen,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			#[codec(crate = $crate::application_crypto::codec)]
			pub struct Public($public);
		}

		impl $crate::application_crypto::CryptoType for Public {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for Public {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `Public` type which is functionally equivalent to `$public` but is
/// new application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_public_common!` must be called too.
/// Can only be used without `full_crypto` feature.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_public_not_full_crypto {
	($public:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(
				Clone, Eq, Hash, PartialEq, Ord, PartialOrd,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::codec::MaxEncodedLen,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			pub struct Public($public);
		}

		impl $crate::application_crypto::CryptoType for Public {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for Public {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;

			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `Public` type which is functionally equivalent to `$public` but is
/// new application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_public_(not)_full_crypto!` must be called too.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_public_common {
	($public:ty, $sig:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::app_crypto_public_common_if_serde!();

		impl AsRef<[u8]> for Public {
			fn as_ref(&self) -> &[u8] {
				self.0.as_ref()
			}
		}

		impl AsMut<[u8]> for Public {
			fn as_mut(&mut self) -> &mut [u8] {
				self.0.as_mut()
			}
		}

		impl $crate::application_crypto::ByteArray for Public {
			const LEN: usize = <$public>::LEN;
		}

		impl $crate::application_crypto::Public for Public {}

		impl $crate::application_crypto::AppPublic for Public {
			type Generic = $public;
		}

		impl<'a> TryFrom<&'a [u8]> for Public {
			type Error = ();

			fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
				<$public>::try_from(data).map(Into::into)
			}
		}

		impl Public {
			/// Convert into wrapped generic public key type.
			pub fn into_inner(self) -> $public {
				self.0
			}
		}
	};
}

#[doc(hidden)]
pub mod module_format_string_prelude {
	#[cfg(all(not(feature = "std"), feature = "serde"))]
	pub use alloc::{format, string::String};
	#[cfg(feature = "std")]
	pub use std::{format, string::String};
}

/// Implements traits for the public key type if `feature = "serde"` is enabled.
#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_public_common_if_serde {
	() => {
		impl $crate::application_crypto::Derive for Public {
			fn derive<Iter: Iterator<Item = $crate::application_crypto::DeriveJunction>>(
				&self,
				path: Iter,
			) -> Option<Self> {
				self.0.derive(path).map(Self)
			}
		}

		impl ::core::fmt::Display for Public {
			fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
				use $crate::application_crypto::Ss58Codec;
				write!(f, "{}", self.0.to_ss58check())
			}
		}

		impl $crate::application_crypto::serde::Serialize for Public {
			fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
			where
				S: $crate::application_crypto::serde::Serializer,
			{
				use $crate::application_crypto::Ss58Codec;
				serializer.serialize_str(&self.to_ss58check())
			}
		}

		impl<'de> $crate::application_crypto::serde::Deserialize<'de> for Public {
			fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
			where
				D: $crate::application_crypto::serde::Deserializer<'de>,
			{
				use $crate::application_crypto::{module_format_string_prelude::*, Ss58Codec};

				Public::from_ss58check(&String::deserialize(deserializer)?)
					.map_err(|e| $crate::application_crypto::serde::de::Error::custom(format!("{:?}", e)))
			}
		}
	};
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_public_common_if_serde {
	() => {
		impl $crate::application_crypto::Derive for Public {}

		impl ::core::fmt::Display for Public {
			fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
				let bytes: &[u8] = self.as_ref();
				for byte in bytes {
					write!(f, "{:02x}", byte)?;
				}
				Ok(())
			}
		}
	};
}

/// Declares Signature type which is functionally equivalent to `$sig`, but is new
/// Application-specific type whose identifier is `$key_type`.
/// For full functionality, app_crypto_public_common! must be called too.
/// Can only be used with `full_crypto` feature
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_signature_full_crypto {
	($sig:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(Clone, Eq, PartialEq,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			#[derive(Hash)]
			pub struct Signature($sig);
		}

		impl $crate::application_crypto::CryptoType for Signature {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for Signature {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `Signature` type which is functionally equivalent to `$sig`, but is new
/// application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_signature_common` must be called too.
/// Can only be used without `full_crypto` feature.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_signature_not_full_crypto {
	($sig:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(Clone, Eq, PartialEq,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			pub struct Signature($sig);
		}

		impl $crate::application_crypto::CryptoType for Signature {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for Signature {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `Signature` type which is functionally equivalent to `$sig`, but is new
/// application-specific type whose identifier is `$key_type`.
/// For full functionality, app_crypto_signature_(not)_full_crypto! must be called too.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_signature_common {
	($sig:ty, $key_type:expr) => {
		impl $crate::application_crypto::Deref for Signature {
			type Target = [u8];

			fn deref(&self) -> &Self::Target {
				self.0.as_ref()
			}
		}

		impl AsRef<[u8]> for Signature {
			fn as_ref(&self) -> &[u8] {
				self.0.as_ref()
			}
		}

		impl AsMut<[u8]> for Signature {
			fn as_mut(&mut self) -> &mut [u8] {
				self.0.as_mut()
			}
		}

		impl $crate::application_crypto::AppSignature for Signature {
			type Generic = $sig;
		}

		impl<'a> TryFrom<&'a [u8]> for Signature {
			type Error = ();

			fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
				<$sig>::try_from(data).map(Into::into)
			}
		}

		impl TryFrom<$crate::application_crypto::Vec<u8>> for Signature {
			type Error = ();

			fn try_from(data: $crate::application_crypto::Vec<u8>) -> Result<Self, Self::Error> {
				Self::try_from(&data[..])
			}
		}

		impl $crate::application_crypto::Signature for Signature {}

		impl $crate::application_crypto::ByteArray for Signature {
			const LEN: usize = <$sig>::LEN;
		}

		impl Signature {
			/// Convert into wrapped generic signature type.
			pub fn into_inner(self) -> $sig {
				self.0
			}
		}
	};
}

/// Declares ProofOfPossession type which is functionally equivalent to `$sig`, but is new
/// Application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_proof_of_possession_common` must be called too.
/// Can only be used with `full_crypto` feature
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_proof_of_possession_full_crypto {
	($sig:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(Clone, Eq, PartialEq, Hash,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			pub struct ProofOfPossession($sig);
		}

		impl $crate::application_crypto::CryptoType for ProofOfPossession {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for ProofOfPossession {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `ProofOfPossession` type which is functionally equivalent to `$sig`, but is new
/// application-specific type whose identifier is `$key_type`.
/// For full functionality, `app_crypto_proof_of_possession_common` must be called too.
/// Can only be used without `full_crypto` feature.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_proof_of_possession_not_full_crypto {
	($sig:ty, $key_type:expr, $crypto_type:expr) => {
		$crate::wrap! {
			/// A generic `AppPublic` wrapper type over $public crypto; this has no specific App.
			#[derive(Clone, Eq, PartialEq,
				$crate::application_crypto::codec::Encode,
				$crate::application_crypto::codec::Decode,
				$crate::application_crypto::codec::DecodeWithMemTracking,
				Debug,
				$crate::application_crypto::scale_info::TypeInfo,
			)]
			pub struct ProofOfPossession($sig);
		}

		impl $crate::application_crypto::CryptoType for ProofOfPossession {
			type Pair = Pair;
		}

		impl $crate::application_crypto::AppCrypto for ProofOfPossession {
			type Public = Public;
			type Pair = Pair;
			type Signature = Signature;
			type ProofOfPossession = ProofOfPossession;
			const ID: $crate::application_crypto::KeyTypeId = $key_type;
			const CRYPTO_ID: $crate::application_crypto::CryptoTypeId = $crypto_type;
		}
	};
}

/// Declares `ProofOfPossession` type which is functionally equivalent to `$sig`, but is new
/// application-specific type whose identifier is `$key_type`.
/// For full functionality, app_crypto_proof_of_possession_(not)_full_crypto! must be called too.
#[doc(hidden)]
#[macro_export]
macro_rules! app_crypto_proof_of_possession_common {
	($sig:ty, $key_type:expr) => {
		impl $crate::application_crypto::Deref for ProofOfPossession {
			type Target = [u8];

			fn deref(&self) -> &Self::Target {
				self.0.as_ref()
			}
		}

		impl AsRef<[u8]> for ProofOfPossession {
			fn as_ref(&self) -> &[u8] {
				self.0.as_ref()
			}
		}

		impl AsMut<[u8]> for ProofOfPossession {
			fn as_mut(&mut self) -> &mut [u8] {
				self.0.as_mut()
			}
		}

		impl $crate::application_crypto::AppSignature for ProofOfPossession {
			type Generic = $sig;
		}

		impl<'a> TryFrom<&'a [u8]> for ProofOfPossession {
			type Error = ();

			fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
				<$sig>::try_from(data).map(Into::into)
			}
		}

		impl TryFrom<$crate::application_crypto::Vec<u8>> for ProofOfPossession {
			type Error = ();

			fn try_from(data: $crate::application_crypto::Vec<u8>) -> Result<Self, Self::Error> {
				Self::try_from(&data[..])
			}
		}

		impl $crate::application_crypto::Signature for ProofOfPossession {}

		impl $crate::application_crypto::ByteArray for ProofOfPossession {
			const LEN: usize = <$sig>::LEN;
		}

		impl ProofOfPossession {
			/// Convert into wrapped generic signature type.
			pub fn into_inner(self) -> $sig {
				self.0
			}
		}
	};
}

/// Implement bidirectional `From` and on-way `AsRef`/`AsMut` for two types, `$inner` and `$outer`.
///
/// ```rust
/// subsoil::wrap! {
///     pub struct Wrapper(u32);
/// }
/// ```
#[macro_export]
macro_rules! wrap {
	($( #[ $attr:meta ] )* struct $outer:ident($inner:ty);) => {
		$( #[ $attr ] )*
		struct $outer( $inner );
		$crate::wrap!($inner, $outer);
	};
	($( #[ $attr:meta ] )* pub struct $outer:ident($inner:ty);) => {
		$( #[ $attr ] )*
		pub struct $outer( $inner );
		$crate::wrap!($inner, $outer);
	};
	($inner:ty, $outer:ty) => {
		impl $crate::application_crypto::Wraps for $outer {
			type Inner = $inner;
		}
		impl From<$inner> for $outer {
			fn from(inner: $inner) -> Self {
				Self(inner)
			}
		}
		impl From<$outer> for $inner {
			fn from(outer: $outer) -> Self {
				outer.0
			}
		}
		impl AsRef<$inner> for $outer {
			fn as_ref(&self) -> &$inner {
				&self.0
			}
		}
		impl AsMut<$inner> for $outer {
			fn as_mut(&mut self) -> &mut $inner {
				&mut self.0
			}
		}
	}
}

/// Generate the given code if the pair type is available.
///
/// The pair type is available when `feature = "std"` || `feature = "full_crypto"`.
///
/// # Example
///
/// ```
/// subsoil::with_pair! {
///     pub type Pair = ();
/// }
/// ```
#[macro_export]
#[cfg(any(feature = "std", feature = "full_crypto"))]
macro_rules! with_pair {
	( $( $def:tt )* ) => {
		$( $def )*
	}
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(not(feature = "std"), not(feature = "full_crypto")))]
macro_rules! with_pair {
	( $( $def:tt )* ) => {};
}
