// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::super::LOG_TARGET;
use subsoil::io::hashing::twox_128;

use topsoil_core::{
	traits::{
		Get, GetStorageVersion, PalletInfoAccess, StorageVersion,
		STORAGE_VERSION_STORAGE_KEY_POSTFIX,
	},
	weights::Weight,
};

/// Migrate the entire storage of this pallet to a new prefix.
///
/// This new prefix must be the same as the one set in construct_runtime. For safety, use
/// `PalletInfo` to get it, as:
/// `<Runtime as topsoil_core::system::Config>::PalletInfo::name::<MembershipPallet>`.
///
/// The migration will look into the storage version in order not to trigger a migration on an up
/// to date storage. Thus the on chain storage version must be less than 4 in order to trigger the
/// migration.
pub fn migrate<
	T: topsoil_core::system::Config,
	P: GetStorageVersion + PalletInfoAccess,
	N: AsRef<str>,
>(
	old_pallet_name: N,
	new_pallet_name: N,
) -> Weight {
	let old_pallet_name = old_pallet_name.as_ref();
	let new_pallet_name = new_pallet_name.as_ref();

	if new_pallet_name == old_pallet_name {
		log::info!(
			target: LOG_TARGET,
			"New pallet name is equal to the old prefix. No migration needs to be done.",
		);
		return Weight::zero();
	}

	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	log::info!(
		target: LOG_TARGET,
		"Running migration to v4 for membership with storage version {:?}",
		on_chain_storage_version,
	);

	if on_chain_storage_version < 4 {
		topsoil_core::storage::migration::move_pallet(
			old_pallet_name.as_bytes(),
			new_pallet_name.as_bytes(),
		);
		log_migration("migration", old_pallet_name, new_pallet_name);

		StorageVersion::new(4).put::<P>();
		<T as topsoil_core::system::Config>::BlockWeights::get().max_block
	} else {
		log::warn!(
			target: LOG_TARGET,
			"Attempted to apply migration to v4 but failed because storage version is {:?}",
			on_chain_storage_version,
		);
		Weight::zero()
	}
}

/// Some checks prior to migration. This can be linked to
/// `topsoil_core::traits::OnRuntimeUpgrade::pre_upgrade` for further testing.
///
/// Panics if anything goes wrong.
pub fn pre_migrate<P: GetStorageVersion, N: AsRef<str>>(old_pallet_name: N, new_pallet_name: N) {
	let old_pallet_name = old_pallet_name.as_ref();
	let new_pallet_name = new_pallet_name.as_ref();
	log_migration("pre-migration", old_pallet_name, new_pallet_name);

	if new_pallet_name == old_pallet_name {
		return;
	}

	let new_pallet_prefix = twox_128(new_pallet_name.as_bytes());
	let storage_version_key = twox_128(STORAGE_VERSION_STORAGE_KEY_POSTFIX);

	let mut new_pallet_prefix_iter = topsoil_core::storage::KeyPrefixIterator::new(
		new_pallet_prefix.to_vec(),
		new_pallet_prefix.to_vec(),
		|key| Ok(key.to_vec()),
	);

	// Ensure nothing except maybe the storage_version_key is stored in the new prefix.
	assert!(new_pallet_prefix_iter.all(|key| key == storage_version_key));

	assert!(<P as GetStorageVersion>::on_chain_storage_version() < 4);
}

/// Some checks for after migration. This can be linked to
/// `topsoil_core::traits::OnRuntimeUpgrade::post_upgrade` for further testing.
///
/// Panics if anything goes wrong.
pub fn post_migrate<P: GetStorageVersion, N: AsRef<str>>(old_pallet_name: N, new_pallet_name: N) {
	let old_pallet_name = old_pallet_name.as_ref();
	let new_pallet_name = new_pallet_name.as_ref();
	log_migration("post-migration", old_pallet_name, new_pallet_name);

	if new_pallet_name == old_pallet_name {
		return;
	}

	// Assert that nothing remains at the old prefix.
	let old_pallet_prefix = twox_128(old_pallet_name.as_bytes());
	let old_pallet_prefix_iter = topsoil_core::storage::KeyPrefixIterator::new(
		old_pallet_prefix.to_vec(),
		old_pallet_prefix.to_vec(),
		|_| Ok(()),
	);
	assert_eq!(old_pallet_prefix_iter.count(), 0);

	// NOTE: storage_version_key is already in the new prefix.
	let new_pallet_prefix = twox_128(new_pallet_name.as_bytes());
	let new_pallet_prefix_iter = topsoil_core::storage::KeyPrefixIterator::new(
		new_pallet_prefix.to_vec(),
		new_pallet_prefix.to_vec(),
		|_| Ok(()),
	);
	assert!(new_pallet_prefix_iter.count() >= 1);

	assert_eq!(<P as GetStorageVersion>::on_chain_storage_version(), 4);
}

fn log_migration(stage: &str, old_pallet_name: &str, new_pallet_name: &str) {
	log::info!(
		target: LOG_TARGET,
		"{}, prefix: '{}' ==> '{}'",
		stage,
		old_pallet_name,
		new_pallet_name,
	);
}
