// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_core::pallet_prelude::*;
use topsoil_core::system::pallet_prelude::*;

pub trait WeightInfo {
	fn foo() -> Weight;
}

impl WeightInfo for () {
	fn foo() -> Weight {
		Weight::zero()
	}
}

#[topsoil_core::pallet]
pub mod parentheses {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	// Crazy man just uses `()`, but it still works ;)
	#[pallet::call(weight(()))]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn foo(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

#[topsoil_core::pallet]
pub mod assign {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
	}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	// Crazy man just uses `()`, but it still works ;)
	#[pallet::call(weight = ())]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn foo(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

fn main() {
}
