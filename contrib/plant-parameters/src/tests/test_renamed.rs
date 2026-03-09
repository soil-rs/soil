// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(any(test, feature = "runtime-benchmarks"))]

//! Tests that the runtime params can be renamed.

use topsoil_core::{
	assert_noop, assert_ok, construct_runtime, derive_impl,
	dynamic_params::{dynamic_pallet_params, dynamic_params},
	traits::AsEnsureOriginWithArg,
};
use topsoil_core::system::EnsureRoot;

use crate as plant_parameters;
use crate::*;
use dynamic_params::*;
use RuntimeParametersRenamed::*;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type Block = topsoil_core::system::mocking::MockBlock<Runtime>;
	type AccountData = plant_balances::AccountData<<Self as plant_balances::Config>::Balance>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
	type AccountStore = System;
}

#[dynamic_params(RuntimeParametersRenamed, plant_parameters::Parameters::<Runtime>)]
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
}

#[cfg(feature = "runtime-benchmarks")]
impl Default for RuntimeParametersRenamed {
	fn default() -> Self {
		RuntimeParametersRenamed::Pallet1(dynamic_params::pallet1::Parameters::Key1(
			dynamic_params::pallet1::Key1,
			Some(123),
		))
	}
}

#[derive_impl(plant_parameters::config_preludes::TestDefaultConfig)]
impl Config for Runtime {
	type AdminOrigin = AsEnsureOriginWithArg<EnsureRoot<Self::AccountId>>;
	type RuntimeParameters = RuntimeParametersRenamed;
	// RuntimeEvent is injected by the `derive_impl` macro.
	// WeightInfo is injected by the `derive_impl` macro.
}

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

#[test]
fn set_parameters_example() {
	new_test_ext().execute_with(|| {
		assert_eq!(pallet1::Key3::get(), 2, "Default works");

		// This gets rejected since the origin is not root.
		assert_noop!(
			PalletParameters::set_parameter(
				RuntimeOrigin::signed(1),
				Pallet1(pallet1::Parameters::Key3(pallet1::Key3, Some(123))),
			),
			DispatchError::BadOrigin
		);

		assert_ok!(PalletParameters::set_parameter(
			RuntimeOrigin::root(),
			Pallet1(pallet1::Parameters::Key3(pallet1::Key3, Some(123))),
		));

		assert_eq!(pallet1::Key3::get(), 123, "Update works");
		assert_last_event(
			crate::Event::Updated {
				key: RuntimeParametersRenamedKey::Pallet1(pallet1::ParametersKey::Key3(
					pallet1::Key3,
				)),
				old_value: None,
				new_value: Some(RuntimeParametersRenamedValue::Pallet1(
					pallet1::ParametersValue::Key3(123),
				)),
			}
			.into(),
		);
	});
}

#[test]
fn get_through_external_pallet_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(<Runtime as plant_example_basic::Config>::MagicNumber::get(), 0);

		assert_ok!(PalletParameters::set_parameter(
			RuntimeOrigin::root(),
			Pallet1(pallet1::Parameters::Key1(pallet1::Key1, Some(123))),
		));

		assert_eq!(<Runtime as plant_example_basic::Config>::MagicNumber::get(), 123);
	});
}
