// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::LOG_TARGET;
use subsoil::io::hashing::twox_128;
use topsoil_core::{
	traits::{Get, StorageVersion},
	weights::Weight,
};

/// The old prefix.
pub const OLD_PREFIX: &[u8] = b"GrandpaFinality";

/// Migrate the entire storage of this pallet to a new prefix.
///
/// This new prefix must be the same as the one set in construct_runtime. For safety, use
/// `PalletInfo` to get it, as:
/// `<Runtime as topsoil_core::system::Config>::PalletInfo::name::<GrandpaPallet>`.
///
/// The old storage prefix, `GrandpaFinality` is hardcoded in the migration code.
pub fn migrate<T: crate::Config, N: AsRef<str>>(new_pallet_name: N) -> Weight {
	if new_pallet_name.as_ref().as_bytes() == OLD_PREFIX {
		log::info!(
			target: LOG_TARGET,
			"New pallet name is equal to the old prefix. No migration needs to be done.",
		);
		return Weight::zero();
	}
	let storage_version = StorageVersion::get::<crate::Pallet<T>>();
	log::info!(
		target: LOG_TARGET,
		"Running migration to v3.1 for grandpa with storage version {:?}",
		storage_version,
	);

	if storage_version <= 3 {
		log::info!("new prefix: {}", new_pallet_name.as_ref());
		topsoil_core::storage::migration::move_pallet(
			OLD_PREFIX,
			new_pallet_name.as_ref().as_bytes(),
		);

		StorageVersion::new(4).put::<crate::Pallet<T>>();

		<T as topsoil_core::system::Config>::BlockWeights::get().max_block
	} else {
		Weight::zero()
	}
}

/// Some checks prior to migration. This can be linked to
/// `topsoil_core::traits::OnRuntimeUpgrade::pre_upgrade` for further testing.
///
/// Panics if anything goes wrong.
pub fn pre_migration<T: crate::Config, N: AsRef<str>>(new: N) {
	let new = new.as_ref();
	log::info!("pre-migration grandpa test with new = {}", new);

	// the next key must exist, and start with the hash of `OLD_PREFIX`.
	let next_key = subsoil::io::storage::next_key(&twox_128(OLD_PREFIX)).unwrap();
	assert!(next_key.starts_with(&twox_128(OLD_PREFIX)));

	// The pallet version is already stored using the pallet name
	let storage_key = StorageVersion::storage_key::<crate::Pallet<T>>();

	// ensure nothing is stored in the new prefix.
	assert!(
		subsoil::io::storage::next_key(&twox_128(new.as_bytes())).map_or(
			// either nothing is there
			true,
			// or we ensure that it has no common prefix with twox_128(new),
			// or isn't the pallet version that is already stored using the pallet name
			|next_key| {
				!next_key.starts_with(&twox_128(new.as_bytes())) || next_key == storage_key
			},
		),
		"unexpected next_key({}) = {:?}",
		new,
		subsoil::core::hexdisplay::HexDisplay::from(
			&subsoil::io::storage::next_key(&twox_128(new.as_bytes())).unwrap()
		),
	);
	// ensure storage version is 3.
	assert_eq!(StorageVersion::get::<crate::Pallet<T>>(), 3);
}

/// Some checks for after migration. This can be linked to
/// `topsoil_core::traits::OnRuntimeUpgrade::post_upgrade` for further testing.
///
/// Panics if anything goes wrong.
pub fn post_migration() {
	log::info!("post-migration grandpa");

	// Assert that nothing remains at the old prefix
	assert!(subsoil::io::storage::next_key(&twox_128(OLD_PREFIX))
		.map_or(true, |next_key| !next_key.starts_with(&twox_128(OLD_PREFIX))));
}
