// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::pallet_prelude::*;
use topsoil_core::system::pallet_prelude::*;

#[topsoil_core::pallet]
pub mod my_test {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[cfg(all(target_endian = "little", target_endian = "big"))] // Never compiles.
	fn never_compiled() {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[cfg(all(target_endian = "little", target_endian = "big"))] // Never compiles.
		#[pallet::weight(Weight::zero())]
		#[pallet::authorize(|_source| {
			never_compiled(); // This will fail to compile if the authorize function is defined.
			Err(InvalidTransaction::Call.into())
		})]
		#[pallet::weight_of_authorize(Weight::zero())]
		#[pallet::call_index(0)]
		pub fn call_0(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(Weight::zero())]
		#[pallet::authorize(|_source| { Err(InvalidTransaction::Call.into()) })]
		#[pallet::weight_of_authorize(Weight::zero())]
		#[pallet::call_index(1)]
		pub fn call_1(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(Weight::zero())]
		#[pallet::call_index(2)]
		pub fn call_2(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

fn main() {}
