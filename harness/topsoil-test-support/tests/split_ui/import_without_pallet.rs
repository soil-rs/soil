// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_support::pallet_macros::*;

#[pallet_section]
mod storages {
	#[pallet::storage]
	pub type MyStorageMap<T: Config> = StorageMap<_, _, u32, u64>;
}

#[import_section(storages)]
pub mod pallet {
	
}

fn main() {
}
