// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

pub mod v1 {
	use crate::{types::BalanceOf, *};
	use alloc::vec::Vec;
	use subsoil::staking::EraIndex;
	use topsoil_support::{
		storage::unhashed,
		traits::{Defensive, Get, GetStorageVersion, OnRuntimeUpgrade},
		weights::Weight,
	};

	#[cfg(feature = "try-runtime")]
	use subsoil::runtime::TryRuntimeError;
	#[cfg(feature = "try-runtime")]
	use topsoil_support::ensure;

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
				// update the version nonetheless.
				current.put::<Pallet<T>>();

				// if a head exists, then we put them back into the queue.
				if Head::<T>::exists() {
					if let Some((stash, _, deposit)) =
						unhashed::take::<(T::AccountId, Vec<EraIndex>, BalanceOf<T>)>(
							&Head::<T>::hashed_key(),
						)
						.defensive()
					{
						Queue::<T>::insert(stash, deposit);
					} else {
						// not much we can do here -- head is already deleted.
					}
					T::DbWeight::get().reads_writes(2, 3)
				} else {
					T::DbWeight::get().reads(2)
				}
			} else {
				log!(info, "Migration did not execute. This probably should be removed");
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			ensure!(
				Pallet::<T>::on_chain_storage_version() == 0,
				"The onchain storage version must be zero for the migration to execute."
			);
			Ok(Default::default())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_: Vec<u8>) -> Result<(), TryRuntimeError> {
			ensure!(
				Pallet::<T>::on_chain_storage_version() == 1,
				"The onchain version must be updated after the migration."
			);
			Ok(())
		}
	}
}
