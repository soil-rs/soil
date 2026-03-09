// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	use codec::{Decode, DecodeWithMemTracking, Encode};
	use topsoil_core::pallet_prelude::{DispatchResultWithPostInfo, Hooks};
	use topsoil_core::system::pallet_prelude::{BlockNumberFor, OriginFor};

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[derive(Encode, Decode, DecodeWithMemTracking, scale_info::TypeInfo, PartialEq, Clone)]
	struct Bar;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn foo(origin: OriginFor<T>, _bar: Bar) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}

fn main() {}
