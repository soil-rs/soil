// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet(dev_mode)]
pub mod pallet {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Something
		Something(u32),
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		phantom: PhantomData<(T, I)>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {}
	}

	#[pallet::error]
	#[derive(PartialEq, Eq)]
	pub enum Error<T, I = ()> {
		/// doc comment put into metadata
		InsufficientProposersBalance,
		NonExistentStorageValue,
	}
}

#[topsoil_core::pallet]
pub mod pallet2 {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Something
		Something(u32),
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		phantom: PhantomData<(T, I)>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {}
	}

	#[pallet::error]
	#[derive(PartialEq, Eq)]
	pub enum Error<T, I = ()> {
		/// doc comment put into metadata
		OtherInsufficientProposersBalance,
		OtherNonExistentStorageValue,
	}
}

#[topsoil_core::pallet]
pub mod pallet3 {
	use topsoil_core::pallet_prelude::*;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Something
		Something(u32),
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		phantom: PhantomData<(T, I)>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {}
	}
}
