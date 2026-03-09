// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
pub mod pallet {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config(with_default, without_automatic_metadata)]
	pub trait Config: topsoil_core::system::Config {
		#[pallet::constant]
		type MyGetParam2: Get<Self::AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}

fn main() {}
