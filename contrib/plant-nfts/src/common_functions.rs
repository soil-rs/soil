// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Various pieces of common functionality.

use crate::*;
use alloc::vec::Vec;
use topsoil_support::pallet_prelude::*;

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Get the owner of the item, if the item exists.
	pub fn owner(collection: T::CollectionId, item: T::ItemId) -> Option<T::AccountId> {
		Item::<T, I>::get(collection, item).map(|i| i.owner)
	}

	/// Get the owner of the collection, if the collection exists.
	pub fn collection_owner(collection: T::CollectionId) -> Option<T::AccountId> {
		Collection::<T, I>::get(collection).map(|i| i.owner)
	}

	/// Validates the signature of the given data with the provided signer's account ID.
	///
	/// # Errors
	///
	/// This function returns a [`WrongSignature`](crate::Error::WrongSignature) error if the
	/// signature is invalid or the verification process fails.
	pub fn validate_signature(
		data: &Vec<u8>,
		signature: &T::OffchainSignature,
		signer: &T::AccountId,
	) -> DispatchResult {
		if signature.verify(&**data, &signer) {
			return Ok(());
		}

		// NOTE: for security reasons modern UIs implicitly wrap the data requested to sign into
		// <Bytes></Bytes>, that's why we support both wrapped and raw versions.
		let prefix = b"<Bytes>";
		let suffix = b"</Bytes>";
		let mut wrapped: Vec<u8> = Vec::with_capacity(data.len() + prefix.len() + suffix.len());
		wrapped.extend(prefix);
		wrapped.extend(data);
		wrapped.extend(suffix);

		ensure!(signature.verify(&*wrapped, &signer), Error::<T, I>::WrongSignature);

		Ok(())
	}

	pub(crate) fn set_next_collection_id(collection: T::CollectionId) {
		let next_id = collection.increment();
		NextCollectionId::<T, I>::set(next_id);
		Self::deposit_event(Event::NextCollectionIdIncremented { next_id });
	}

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	pub fn set_next_id(id: T::CollectionId) {
		NextCollectionId::<T, I>::set(Some(id));
	}

	#[cfg(test)]
	pub fn get_next_id() -> T::CollectionId {
		NextCollectionId::<T, I>::get()
			.or(T::CollectionId::initial_value())
			.expect("Failed to get next collection ID")
	}
}
