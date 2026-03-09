// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::pallet_prelude::DispatchResult;
	use topsoil_core::system::pallet_prelude::OriginFor;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
        #[pallet::weight(10_000something)]
		pub fn foo(_: OriginFor<T>) -> DispatchResult { Ok(()) }
	}
}

fn main() {
}
