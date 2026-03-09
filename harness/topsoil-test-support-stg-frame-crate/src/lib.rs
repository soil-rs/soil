// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil::deps::topsoil_core;

#[topsoil_core::pallet]
pub mod pallet {
	use super::*;
	use topsoil_core::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	// The only valid syntax here is the following or
	// ```
	// pub trait Config: topsoil::deps::topsoil_core::system::Config {}
	// ```
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::genesis_config]
	#[derive(topsoil_core::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		#[serde(skip)]
		_config: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {}
	}
}

#[cfg(test)]
// Dummy test to make sure a runtime would compile.
mod tests {
	use super::{
		pallet,
		topsoil_core,
		topsoil_core::{construct_runtime, derive_impl},
	};

	type Block = topsoil_core::system::mocking::MockBlock<Runtime>;

	impl crate::pallet::Config for Runtime {}

	#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
	impl topsoil_core::system::Config for Runtime {
		type Block = Block;
	}

	construct_runtime! {
		pub struct Runtime
		{
			System: topsoil_core::system,
			Pallet: pallet,
		}
	}
}
