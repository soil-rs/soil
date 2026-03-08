// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
mod pallet {
	use topsoil_support::pallet_prelude::{Hooks, IsType};
	use topsoil_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: topsoil_system::Config {
		type Bar: Clone + PartialEq + std::fmt::Debug;
		type RuntimeEvent: IsType<<Self as topsoil_system::Config>::RuntimeEvent> + From<Event<Self>>;
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
