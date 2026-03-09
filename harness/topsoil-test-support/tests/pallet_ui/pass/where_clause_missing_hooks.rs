// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
pub mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_core::system::Config
	where
		<Self as topsoil_core::system::Config>::Nonce: From<u128>,
	{
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> where <T as topsoil_core::system::Config>::Nonce: From<u128> {}

	impl<T: Config> Pallet<T>
	where
		<T as topsoil_core::system::Config>::Nonce: From<u128>,
	{
		pub fn foo(x: u128) {
			let _index = <T as topsoil_core::system::Config>::Nonce::from(x);
		}
	}
}

fn main() {}
