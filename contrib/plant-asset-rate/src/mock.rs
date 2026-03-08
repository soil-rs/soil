// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The crate's mock.

use crate as plant_asset_rate;
use subsoil::runtime::BuildStorage;
use topsoil_support::derive_impl;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		AssetRate: plant_asset_rate,
		Balances: plant_balances,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl plant_asset_rate::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type CreateOrigin = topsoil_system::EnsureRoot<u64>;
	type RemoveOrigin = topsoil_system::EnsureRoot<u64>;
	type UpdateOrigin = topsoil_system::EnsureRoot<u64>;
	type Currency = Balances;
	type AssetKind = u32;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
