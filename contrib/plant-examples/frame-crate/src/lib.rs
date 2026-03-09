// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil::prelude::*;

#[topsoil::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::storage]
	pub type Value<T> = StorageValue<Value = u32>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn some_dispatchable(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::pallet as my_pallet;
	use topsoil::testing_prelude::*;

	construct_runtime!(
		pub enum Runtime {
			System: topsoil_core::system,
			MyPallet: my_pallet,
		}
	);

	#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
	impl topsoil_core::system::Config for Runtime {
		type Block = MockBlock<Self>;
	}

	impl my_pallet::Config for Runtime {}
}
