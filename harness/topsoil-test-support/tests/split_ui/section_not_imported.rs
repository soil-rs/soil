// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_support::pallet_macros::*;

pub use pallet::*;

#[pallet_section]
mod storages {
	#[pallet::storage]
	pub type MyStorageMap<T: Config> = StorageMap<_, _, u32, u64>;
}

#[topsoil_support::pallet(dev_mode)]
pub mod pallet {
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn my_call(_origin: OriginFor<T>) -> DispatchResult {
			MyStorageMap::<T>::insert(1, 2);
			Ok(())
		}
	}
}

fn main() {
}
