// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::{
	traits::{Get, OnRuntimeUpgrade},
	weights::Weight,
};

use crate::{Config, CurrentSetId, SetIdSession, LOG_TARGET};

pub use v5::MigrateV4ToV5;

/// Version 4.
pub mod v4;
mod v5;

/// This migration will clean up all stale set id -> session entries from the
/// `SetIdSession` storage map, only the latest `max_set_id_session_entries`
/// will be kept.
///
/// This migration should be added with a runtime upgrade that introduces the
/// `MaxSetIdSessionEntries` constant to the pallet (although it could also be
/// done later on).
pub struct CleanupSetIdSessionMap<T>(core::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for CleanupSetIdSessionMap<T> {
	fn on_runtime_upgrade() -> Weight {
		// NOTE: since this migration will loop over all stale entries in the
		// map we need to set some cutoff value, otherwise the migration might
		// take too long to run. for scenarios where there are that many entries
		// to cleanup a multiblock migration will be needed instead.
		if CurrentSetId::<T>::get() > 25_000 {
			log::warn!(
				target: LOG_TARGET,
				"CleanupSetIdSessionMap migration was aborted since there are too many entries to cleanup."
			);

			return T::DbWeight::get().reads(1);
		}

		cleanup_set_id_sesion_map::<T>()
	}
}

fn cleanup_set_id_sesion_map<T: Config>() -> Weight {
	let until_set_id = CurrentSetId::<T>::get().saturating_sub(T::MaxSetIdSessionEntries::get());

	for set_id in 0..=until_set_id {
		SetIdSession::<T>::remove(set_id);
	}

	T::DbWeight::get()
		.reads(1)
		.saturating_add(T::DbWeight::get().writes(until_set_id + 1))
}
