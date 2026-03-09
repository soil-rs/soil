// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(any(test, feature = "runtime-benchmarks"))]
#![allow(non_snake_case)]

//! Mock runtime that configures the `plant_example_basic` to use dynamic params for testing.

use topsoil_core::{
	construct_runtime, derive_impl,
	dynamic_params::{dynamic_pallet_params, dynamic_params},
	traits::EnsureOriginWithArg,
};

use crate as plant_parameters;
use crate::*;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Block = topsoil_core::system::mocking::MockBlock<Runtime>;
	type AccountData = plant_balances::AccountData<<Self as plant_balances::Config>::Balance>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
	type AccountStore = System;
}

#[docify::export]
#[dynamic_params(RuntimeParameters, plant_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
	use super::*;

	#[dynamic_pallet_params]
	#[codec(index = 3)]
	pub mod pallet1 {
		#[codec(index = 0)]
		pub static Key1: u64 = 0;
		#[codec(index = 1)]
		pub static Key2: u32 = 1;
		#[codec(index = 2)]
		pub static Key3: u128 = 2;
	}

	#[dynamic_pallet_params]
	#[codec(index = 1)]
	pub mod pallet2 {
		#[codec(index = 2)]
		pub static Key1: u64 = 0;
		#[codec(index = 1)]
		pub static Key2: u32 = 2;
		#[codec(index = 0)]
		pub static Key3: u128 = 4;
	}

	#[dynamic_pallet_params]
	#[codec(index = 2)]
	pub mod nis {
		#[codec(index = 0)]
		pub static Target: u64 = 0;
	}

	#[dynamic_pallet_params]
	#[codec(index = 4)]
	pub mod somE_weird_SPElLInG_s {
		#[codec(index = 0)]
		pub static V: u64 = 0;
	}
}

#[docify::export(benchmarking_default)]
#[cfg(feature = "runtime-benchmarks")]
impl Default for RuntimeParameters {
	fn default() -> Self {
		RuntimeParameters::Pallet1(dynamic_params::pallet1::Parameters::Key1(
			dynamic_params::pallet1::Key1,
			Some(123),
		))
	}
}

#[docify::export]
mod custom_origin {
	use super::*;
	pub struct ParamsManager;

	impl EnsureOriginWithArg<RuntimeOrigin, RuntimeParametersKey> for ParamsManager {
		type Success = ();

		fn try_origin(
			origin: RuntimeOrigin,
			key: &RuntimeParametersKey,
		) -> Result<Self::Success, RuntimeOrigin> {
			// Account 123 is allowed to set parameters in benchmarking only:
			#[cfg(feature = "runtime-benchmarks")]
			if ensure_signed(origin.clone()).is_ok_and(|acc| acc == 123) {
				return Ok(());
			}

			match key {
				RuntimeParametersKey::SomEWeirdSPElLInGS(_)
				| RuntimeParametersKey::Nis(_)
				| RuntimeParametersKey::Pallet1(_) => ensure_root(origin.clone()),
				RuntimeParametersKey::Pallet2(_) => ensure_signed(origin.clone()).map(|_| ()),
			}
			.map_err(|_| origin)
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn try_successful_origin(_key: &RuntimeParametersKey) -> Result<RuntimeOrigin, ()> {
			Ok(RuntimeOrigin::signed(123))
		}
	}
}

#[docify::export(impl_config)]
#[derive_impl(plant_parameters::config_preludes::TestDefaultConfig)]
impl Config for Runtime {
	type AdminOrigin = custom_origin::ParamsManager;
	// RuntimeParameters is injected by the `derive_impl` macro.
	// RuntimeEvent is injected by the `derive_impl` macro.
	// WeightInfo is injected by the `derive_impl` macro.
}

#[docify::export(usage)]
impl plant_example_basic::Config for Runtime {
	// Use the dynamic key in the pallet config:
	type MagicNumber = dynamic_params::pallet1::Key1;
	type WeightInfo = ();
}

construct_runtime!(
	pub enum Runtime {
		System: topsoil_core::system,
		PalletParameters: crate,
		Example: plant_example_basic,
		Balances: plant_balances,
	}
);

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut ext = subsoil::io::TestExternalities::new(Default::default());
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub(crate) fn assert_last_event(generic_event: RuntimeEvent) {
	let events = topsoil_core::system::Pallet::<Runtime>::events();
	// compare to the last event record
	let topsoil_core::system::EventRecord { event, .. } = &events.last().expect("Event expected");
	assert_eq!(event, &generic_event);
}
