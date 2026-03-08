// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Various pieces of common functionality.
use super::*;
use core::marker::PhantomData;
use topsoil_support::traits::{Get, UncheckedOnRuntimeUpgrade};

mod v1 {
	use super::*;

	/// Actual implementation of the storage migration.
	pub struct UncheckedMigrateToV1Impl<T, I>(PhantomData<(T, I)>);

	impl<T: Config<I>, I: 'static> UncheckedOnRuntimeUpgrade for UncheckedMigrateToV1Impl<T, I> {
		fn on_runtime_upgrade() -> topsoil_support::weights::Weight {
			let mut count = 0;
			for (collection, detail) in Collection::<T, I>::iter() {
				CollectionAccount::<T, I>::insert(&detail.owner, &collection, ());
				count += 1;
			}

			log::info!(
				target: LOG_TARGET,
				"Storage migration v1 for uniques finished.",
			);

			// calculate and return migration weights
			T::DbWeight::get().reads_writes(count as u64 + 1, count as u64 + 1)
		}
	}
}

/// Migrate the pallet storage from `0` to `1`.
pub type MigrateV0ToV1<T, I> = topsoil_support::migrations::VersionedMigration<
	0,
	1,
	v1::UncheckedMigrateToV1Impl<T, I>,
	Pallet<T, I>,
	<T as topsoil_system::Config>::DbWeight,
>;
