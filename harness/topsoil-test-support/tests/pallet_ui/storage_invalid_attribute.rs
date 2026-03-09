// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::pallet_prelude::Hooks;
	use topsoil_core::system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	#[pallet::generate_store(pub trait Store)]
	type Foo<T> = StorageValue<u8, u8>;
}

fn main() {
}
