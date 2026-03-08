// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet(dev_mode)]
mod pallet {
	use topsoil_support::pallet_prelude::DispatchResult;
	use topsoil_system::pallet_prelude::OriginFor;

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::feeless_if(|| -> bool { true })]
		pub fn foo(_: OriginFor<T>) -> DispatchResult { Ok(()) }
	}
}

fn main() {
}
