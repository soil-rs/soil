// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;

#[topsoil_core::pallet]
mod pallet {
	use topsoil_core::system::pallet_prelude::*;
	use topsoil_core::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn call_1(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

pub use pallet::*;

#[benchmarks]
mod benches {
	use super::*;
	use topsoil_core::traits::OriginTrait;

	#[benchmark]
	fn call_1() {
		let origin = 3u8;
		#[extrinsic_call]
		_(origin);
	}
}

fn main() {}
