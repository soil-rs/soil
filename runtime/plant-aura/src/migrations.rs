// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Migrations for the AURA pallet.

use topsoil_core::{pallet_prelude::*, traits::Get, weights::Weight};

struct __LastTimestamp<T>(core::marker::PhantomData<T>);
impl<T: RemoveLastTimestamp> topsoil_core::traits::StorageInstance for __LastTimestamp<T> {
	fn pallet_prefix() -> &'static str {
		T::PalletPrefix::get()
	}
	const STORAGE_PREFIX: &'static str = "LastTimestamp";
}

type LastTimestamp<T> = StorageValue<__LastTimestamp<T>, (), ValueQuery>;

pub trait RemoveLastTimestamp: super::Config {
	type PalletPrefix: Get<&'static str>;
}

/// Remove the `LastTimestamp` storage value.
///
/// This storage value was removed and replaced by `CurrentSlot`. As we only remove this storage
/// value, it is safe to call this method multiple times.
///
/// This migration requires a type `T` that implements [`RemoveLastTimestamp`].
pub fn remove_last_timestamp<T: RemoveLastTimestamp>() -> Weight {
	LastTimestamp::<T>::kill();
	T::DbWeight::get().writes(1)
}
