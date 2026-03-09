// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Transaction extension which validates a signature against a payload constructed from a call and
//! the rest of the transaction extension pipeline.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod extension;
#[cfg(test)]
mod tests;
pub mod weights;

extern crate alloc;

#[cfg(feature = "runtime-benchmarks")]
pub use benchmarking::BenchmarkHelper;
use codec::{Decode, Encode};
pub use extension::VerifySignature;
use topsoil_core::Parameter;
pub use weights::WeightInfo;

pub use pallet::*;

#[topsoil_core::pallet]
pub mod pallet {
	use super::*;
	use subsoil::runtime::traits::{IdentifyAccount, Verify};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configuration trait.
	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		/// Signature type that the extension of this pallet can verify.
		type Signature: Verify<Signer = Self::AccountIdentifier>
			+ Parameter
			+ Encode
			+ Decode
			+ Send
			+ Sync;
		/// The account identifier used by this pallet's signature type.
		type AccountIdentifier: IdentifyAccount<AccountId = Self::AccountId>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		/// Helper to create a signature to be benchmarked.
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper: BenchmarkHelper<Self::Signature, Self::AccountId>;
	}
}
