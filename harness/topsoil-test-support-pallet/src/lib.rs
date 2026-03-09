// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! A basic pallet that can be used to test `construct_runtime!`.

// Ensure docs are propagated properly by the macros.
#![warn(missing_docs)]

pub use pallet::*;

#[topsoil_core::pallet]
pub mod pallet {
	use topsoil_core::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	/// I'm the documentation
	#[pallet::storage]
	pub type Value<T> = StorageValue<_, u32>;

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

	#[pallet::error]
	pub enum Error<T> {
		/// Something failed
		Test,
	}
}
