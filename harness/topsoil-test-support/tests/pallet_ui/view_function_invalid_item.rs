// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
mod pallet {
	use topsoil_support::pallet_prelude::*;

	#[pallet::config(with_default)]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::view_functions]
	impl<T: Config> Pallet<T> {
		pub const SECONDS_PER_MINUTE: u32 = 60;
	}
}

fn main() {}
