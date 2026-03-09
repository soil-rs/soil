// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(any(all(feature = "try-runtime", test), doc))]

use crate::*;
use topsoil_core::{derive_impl, weights::constants::ParityDbWeight};

// Re-export crate as its pallet name for construct_runtime.
use crate as pallet_example_storage_migration;

type Block = topsoil_core::system::mocking::MockBlock<MockRuntime>;

// For testing the pallet, we construct a mock runtime.
topsoil_core::construct_runtime!(
	pub struct MockRuntime {
		System: topsoil_core::system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: plant_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Example: pallet_example_storage_migration::{Pallet, Call, Storage},
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for MockRuntime {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
	type DbWeight = ParityDbWeight;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for MockRuntime {
	type AccountStore = System;
}

impl Config for MockRuntime {}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	use subsoil::runtime::BuildStorage;

	let t = RuntimeGenesisConfig { system: Default::default(), balances: Default::default() }
		.build_storage()
		.unwrap();
	t.into()
}
