// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::storage]
	type Foo<T> = StorageValue<_, u8>;

	#[pallet::storage]
	#[pallet::storage_prefix = "Foo"]
	type NotFoo<T> = StorageValue<_, u16>;

	#[pallet::storage]
	type CounterForBar<T> = StorageValue<_, u16>;

	#[pallet::storage]
	type Bar<T> = CountedStorageMap<_, Twox64Concat, u16, u16>;
}

fn main() {
}
