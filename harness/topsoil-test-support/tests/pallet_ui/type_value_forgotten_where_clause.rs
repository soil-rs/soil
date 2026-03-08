// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
mod pallet {
	use topsoil_support::pallet_prelude::Hooks;
	use topsoil_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: topsoil_system::Config
	where <Self as topsoil_system::Config>::AccountId: From<u32>
	{}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where <T as topsoil_system::Config>::AccountId: From<u32>
	{}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where <T as topsoil_system::Config>::AccountId: From<u32>
	{}

	#[pallet::type_value] fn Foo<T: Config>() -> u32 { 3u32 }
}

fn main() {
}
