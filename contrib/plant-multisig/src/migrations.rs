// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::*;
use topsoil::prelude::*;

pub mod v1 {
	use super::*;

	type OpaqueCall<T> = topsoil::traits::WrapperKeepOpaque<<T as Config>::RuntimeCall>;

	#[topsoil::storage_alias]
	type Calls<T: Config> = StorageMap<
		Pallet<T>,
		Identity,
		[u8; 32],
		(OpaqueCall<T>, <T as topsoil_system::Config>::AccountId, BalanceOf<T>),
	>;

	pub struct MigrateToV1<T>(core::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, topsoil::try_runtime::TryRuntimeError> {
			log!(info, "Number of calls to refund and delete: {}", Calls::<T>::iter().count());

			Ok(Vec::new())
		}

		fn on_runtime_upgrade() -> Weight {
			use topsoil::traits::ReservableCurrency as _;
			let current = Pallet::<T>::in_code_storage_version();
			let onchain = Pallet::<T>::on_chain_storage_version();

			if onchain > 0 {
				log!(info, "MigrateToV1 should be removed");
				return T::DbWeight::get().reads(1);
			}

			let mut call_count = 0u64;
			Calls::<T>::drain().for_each(|(_call_hash, (_data, caller, deposit))| {
				T::Currency::unreserve(&caller, deposit);
				call_count.saturating_inc();
			});

			current.put::<Pallet<T>>();

			T::DbWeight::get().reads_writes(
				// Reads: Get Calls + Get Version
				call_count.saturating_add(1),
				// Writes: Drain Calls + Unreserves + Set version
				call_count.saturating_mul(2).saturating_add(1),
			)
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), topsoil::try_runtime::TryRuntimeError> {
			ensure!(
				Calls::<T>::iter().count() == 0,
				"there are some dangling calls that need to be destroyed and refunded"
			);
			Ok(())
		}
	}
}
