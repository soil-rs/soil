// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config(with_default)]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type MyStorage<T> = StorageValue<_, u32>;

	#[pallet::view_functions]
	impl<T: Config> Pallet<T> {
		fn get_value() -> Option<u32> {
			MyStorage::<T>::get()
		}
	}
}

fn main() {}
