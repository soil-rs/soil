// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{BoundedAuthorityList, Pallet};
use alloc::vec::Vec;
use codec::Decode;
use core::marker::PhantomData;
use subsoil::consensus::grandpa::AuthorityList;
use topsoil_support::{
	migrations::VersionedMigration,
	storage,
	traits::{Get, UncheckedOnRuntimeUpgrade},
	weights::Weight,
};

const GRANDPA_AUTHORITIES_KEY: &[u8] = b":grandpa_authorities";

fn load_authority_list() -> AuthorityList {
	storage::unhashed::get_raw(GRANDPA_AUTHORITIES_KEY).map_or_else(
		|| Vec::new(),
		|l| <(u8, AuthorityList)>::decode(&mut &l[..]).unwrap_or_default().1,
	)
}

/// Actual implementation of [`MigrateV4ToV5`].
pub struct UncheckedMigrateImpl<T>(PhantomData<T>);

impl<T: crate::Config> UncheckedOnRuntimeUpgrade for UncheckedMigrateImpl<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, subsoil::runtime::TryRuntimeError> {
		use codec::Encode;

		let authority_list_len = load_authority_list().len() as u32;

		if authority_list_len > T::MaxAuthorities::get() {
			return Err(
				"Grandpa: `Config::MaxAuthorities` is smaller than the actual number of authorities.".into()
			);
		}

		if authority_list_len == 0 {
			return Err("Grandpa: Authority list is empty!".into());
		}

		Ok(authority_list_len.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), subsoil::runtime::TryRuntimeError> {
		let len = u32::decode(&mut &state[..]).unwrap();

		topsoil_support::ensure!(
			len == crate::Pallet::<T>::grandpa_authorities().len() as u32,
			"Grandpa: pre-migrated and post-migrated list should have the same length"
		);

		topsoil_support::ensure!(
			load_authority_list().is_empty(),
			"Old authority list shouldn't exist anymore"
		);

		Ok(())
	}

	fn on_runtime_upgrade() -> Weight {
		crate::Authorities::<T>::put(
			&BoundedAuthorityList::<T::MaxAuthorities>::force_from(
				load_authority_list(),
				Some("Grandpa: `Config::MaxAuthorities` is smaller than the actual number of authorities.")
			)
		);

		storage::unhashed::kill(GRANDPA_AUTHORITIES_KEY);

		T::DbWeight::get().reads_writes(1, 2)
	}
}

/// Migrate the storage from V4 to V5.
///
/// Switches from `GRANDPA_AUTHORITIES_KEY` to a normal FRAME storage item.
pub type MigrateV4ToV5<T> = VersionedMigration<
	4,
	5,
	UncheckedMigrateImpl<T>,
	Pallet<T>,
	<T as topsoil_system::Config>::DbWeight,
>;
