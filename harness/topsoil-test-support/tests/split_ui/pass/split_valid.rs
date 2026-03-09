// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use topsoil_core::pallet_macros::*;

pub use pallet::*;

#[pallet_section]
mod events {
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingDone,
	}
}

#[import_section(events)]
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
	}
}

fn main() {}
