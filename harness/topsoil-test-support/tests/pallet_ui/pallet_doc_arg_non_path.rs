// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
// Must receive a string literal pointing to a path
#[pallet_doc(X)]
mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_system::Config
	where
		<Self as topsoil_system::Config>::Nonce: From<u128>,
	{
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);
}

fn main() {}
