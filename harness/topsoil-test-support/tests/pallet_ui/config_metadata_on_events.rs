// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
mod pallet {
	use topsoil_support::pallet_prelude::*;
	use topsoil_system::pallet_prelude::*;

	#[pallet::config(with_default)]
	pub trait Config: topsoil_system::Config {
		#[pallet::no_default_bounds]
		#[pallet::include_metadata]
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as topsoil_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MyGetParam2: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

fn main() {}
