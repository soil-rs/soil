// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(test)]

use crate as plant_whitelist;

use topsoil::testing_prelude::*;
type Block = MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Whitelist: plant_whitelist,
		Preimage: plant_preimage,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl plant_preimage::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
	type WeightInfo = ();
}

impl plant_whitelist::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WhitelistOrigin = EnsureRoot<Self::AccountId>;
	type DispatchWhitelistedOrigin = EnsureRoot<Self::AccountId>;
	type Preimages = Preimage;
	type WeightInfo = ();
}

pub fn new_test_ext() -> TestExternalities {
	let t = RuntimeGenesisConfig::default().build_storage().unwrap();
	let mut ext = TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
