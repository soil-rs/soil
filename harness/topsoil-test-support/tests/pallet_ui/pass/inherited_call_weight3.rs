// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::pallet_prelude::*;
use topsoil_system::pallet_prelude::*;

// If, for whatever reason, you don't to not use a `WeightInfo` trait - it will still work.
pub struct Impl;

impl Impl {
	pub fn foo() -> Weight {
		Weight::zero()
	}
}

#[topsoil_support::pallet]
pub mod parentheses {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call(weight(crate::Impl))]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn foo(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

#[topsoil_support::pallet]
pub mod assign {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call(weight = crate::Impl)]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn foo(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

fn main() {
}
