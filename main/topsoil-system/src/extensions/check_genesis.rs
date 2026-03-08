// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{pallet_prelude::BlockNumberFor, Config, Pallet};
use codec::{Decode, DecodeWithMemTracking, Encode};
use scale_info::TypeInfo;
use subsoil::runtime::{
	traits::{TransactionExtension, Zero},
	transaction_validity::TransactionValidityError,
};

/// Genesis hash check to provide replay protection between different networks.
///
/// # Transaction Validity
///
/// Note that while a transaction with invalid `genesis_hash` will fail to be decoded,
/// the extension does not affect any other fields of `TransactionValidity` directly.
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckGenesis<T: Config + Send + Sync>(core::marker::PhantomData<T>);

impl<T: Config + Send + Sync> core::fmt::Debug for CheckGenesis<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "CheckGenesis")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> CheckGenesis<T> {
	/// Creates new `TransactionExtension` to check genesis hash.
	pub fn new() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> TransactionExtension<T::RuntimeCall> for CheckGenesis<T> {
	const IDENTIFIER: &'static str = "CheckGenesis";
	type Implicit = T::Hash;
	fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
		Ok(<Pallet<T>>::block_hash(BlockNumberFor::<T>::zero()))
	}
	type Val = ();
	type Pre = ();
	fn weight(&self, _: &T::RuntimeCall) -> subsoil::weights::Weight {
		// All transactions will always read the hash of the genesis block, so to avoid
		// charging this multiple times in a block we manually set the proof size to 0.
		<T::ExtensionsWeightInfo as super::WeightInfo>::check_genesis().set_proof_size(0)
	}
	subsoil::impl_tx_ext_default!(T::RuntimeCall; validate prepare);
}
