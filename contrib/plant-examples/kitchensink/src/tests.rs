// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for topsoil-example-kitchensink.

use crate::*;
use subsoil::runtime::BuildStorage;
use topsoil_core::{assert_ok, derive_impl, parameter_types, traits::VariantCountOf};
// Reexport crate as its pallet name for construct_runtime.
use crate as plant_example_kitchensink;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

// For testing the pallet, we construct a mock runtime.
topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: plant_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Kitchensink: plant_example_kitchensink::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

/// Using a default config for [`topsoil_system`] in tests. See `default-config` example for more
/// details.
#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

parameter_types! {
	pub const InMetadata: u32 = 30;
}

impl Config for Test {
	type WeightInfo = ();

	type Currency = Balances;
	type InMetadata = InMetadata;

	const FOO: u32 = 100;

	fn some_function() -> u32 {
		5u32
	}
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = RuntimeGenesisConfig {
		// We use default for brevity, but you can configure as desired if needed.
		system: Default::default(),
		balances: Default::default(),
		kitchensink: plant_example_kitchensink::GenesisConfig { bar: 32, foo: 24 },
	}
	.build_storage()
	.unwrap();
	t.into()
}

#[test]
fn set_foo_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Foo::<Test>::get(), Some(24)); // From genesis config.

		let val1 = 42;
		assert_ok!(Kitchensink::set_foo(RuntimeOrigin::root(), val1, 2));
		assert_eq!(Foo::<Test>::get(), Some(val1));
	});
}
