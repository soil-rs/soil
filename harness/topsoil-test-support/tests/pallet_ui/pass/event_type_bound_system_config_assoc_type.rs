// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
#[allow(unused_imports)]
pub mod pallet {
	use topsoil_core::pallet_prelude::{Hooks, IsType};
	use topsoil_core::system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config:
		topsoil_core::system::Config<Hash = subsoil::core::H256, RuntimeEvent: From<Event<Self>>>
	{
		type Bar: Clone + std::fmt::Debug + Eq;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::event]
	pub enum Event<T: Config> {
		B { b: T::Bar },
	}
}

fn main() {}
