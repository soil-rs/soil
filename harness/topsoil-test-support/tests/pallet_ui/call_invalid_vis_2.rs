// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::pallet_prelude::{Hooks, DispatchResultWithPostInfo};
	use topsoil_core::system::pallet_prelude::{BlockNumberFor, OriginFor};

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		type Bar: codec::Codec;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub(crate) fn foo(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

fn main() {
}
