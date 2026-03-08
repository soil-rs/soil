// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;

#[topsoil_support::pallet]
mod pallet {
	use topsoil_system::pallet_prelude::*;
	use topsoil_support::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

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
	use topsoil_support::traits::OriginTrait;

	#[benchmark]
	fn call_1() {
		let origin = 3u8;
		#[extrinsic_call]
		_(origin);
	}
}

fn main() {}
