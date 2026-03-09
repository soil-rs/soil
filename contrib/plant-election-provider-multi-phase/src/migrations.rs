// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

pub mod v1 {
	use alloc::collections::btree_map::BTreeMap;
	use topsoil_core::{
		storage::unhashed,
		traits::{Defensive, GetStorageVersion, OnRuntimeUpgrade},
		BoundedVec,
	};

	use crate::*;
	pub struct MigrateToV1<T>(core::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			let current = Pallet::<T>::in_code_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			log!(
				info,
				"Running migration with in-code storage version {:?} / onchain {:?}",
				current,
				onchain
			);

			if current == 1 && onchain == 0 {
				if SignedSubmissionIndices::<T>::exists() {
					// This needs to be tested at a both a block height where this value exists, and
					// when it doesn't.
					let now = topsoil_core::system::Pallet::<T>::block_number();
					let map = unhashed::get::<BTreeMap<ElectionScore, u32>>(
						&SignedSubmissionIndices::<T>::hashed_key(),
					)
					.defensive_unwrap_or_default();
					let vector = map
						.into_iter()
						.map(|(score, index)| (score, now, index))
						.collect::<Vec<_>>();

					log!(
						debug,
						"{:?} SignedSubmissionIndices read from storage (max: {:?})",
						vector.len(),
						T::SignedMaxSubmissions::get()
					);

					// defensive-only, assuming a constant `SignedMaxSubmissions`.
					let bounded = BoundedVec::<_, _>::truncate_from(vector);
					SignedSubmissionIndices::<T>::put(bounded);

					log!(info, "SignedSubmissionIndices existed and got migrated");
				} else {
					log!(info, "SignedSubmissionIndices did NOT exist.");
				}

				current.put::<Pallet<T>>();
				T::DbWeight::get().reads_writes(2, 1)
			} else {
				log!(info, "Migration did not execute. This probably should be removed");
				T::DbWeight::get().reads(1)
			}
		}
	}
}
