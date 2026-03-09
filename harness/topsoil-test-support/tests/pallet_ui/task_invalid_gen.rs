// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet(dev_mode)]
mod pallet_with_instance {
	use topsoil_core::pallet_prelude::{ValueQuery, StorageValue};

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::storage]
	pub type SomeStorage<T, I = ()> = StorageValue<_, u32, ValueQuery>;

	#[pallet::task_enum]
	pub enum Task<T> {}

	#[pallet::tasks_experimental]
	impl topsoil_core::traits::Task for Task {}
}

fn main() {
}
