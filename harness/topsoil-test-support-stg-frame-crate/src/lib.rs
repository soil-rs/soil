// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil::deps::{topsoil_support, topsoil_system};

#[topsoil_support::pallet]
pub mod pallet {
	use super::*;
	use topsoil_support::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	// The only valid syntax here is the following or
	// ```
	// pub trait Config: topsoil::deps::topsoil_system::Config {}
	// ```
	pub trait Config: topsoil_system::Config {}

	#[pallet::genesis_config]
	#[derive(topsoil_support::DefaultNoBound)]
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
		topsoil_support::{construct_runtime, derive_impl},
		topsoil_system,
	};

	type Block = topsoil_system::mocking::MockBlock<Runtime>;

	impl crate::pallet::Config for Runtime {}

	#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
	impl topsoil_system::Config for Runtime {
		type Block = Block;
	}

	construct_runtime! {
		pub struct Runtime
		{
			System: topsoil_system,
			Pallet: pallet,
		}
	}
}
