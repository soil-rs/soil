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
		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call(weight = T::WeightInfo)]
	impl<T: Config> Pallet<T> {
		#[pallet::authorize(|_, _: u8| -> bool { true })]
		#[pallet::weight_of_authorize(Weight::zero())]
		#[pallet::weight(Weight::zero())]
		#[pallet::call_index(0)]
		pub fn call1(origin: OriginFor<T>, a: u32) -> DispatchResult {
			let _ = origin;
			let _ = a;
			Ok(())
		}
	}
}

fn main() {}
