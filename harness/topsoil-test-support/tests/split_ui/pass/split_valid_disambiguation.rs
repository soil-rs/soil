// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_core::pallet_macros::*;

pub use pallet::*;

mod first {
	use super::*;

	#[pallet_section]
	mod section {
		#[pallet::event]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> {
			SomethingDone,
		}
	}
}

mod second {
	use super::*;

	#[pallet_section(section2)]
	mod section {
		#[pallet::error]
		pub enum Error<T> {
			NoneValue,
		}
	}
}

#[import_section(first::section)]
#[import_section(second::section2)]
#[topsoil_core::pallet(dev_mode)]
pub mod pallet {
	use topsoil_core::pallet_prelude::*;
	use topsoil_core::system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn my_call(_origin: OriginFor<T>) -> DispatchResult {
			Self::deposit_event(Event::SomethingDone);
			Ok(())
		}

		pub fn my_call_2(_origin: OriginFor<T>) -> DispatchResult {
			return Err(Error::<T>::NoneValue.into())
		}
	}
}

fn main() {}
