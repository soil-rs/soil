// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

#![cfg(test)]

use crate::{self as plant_indices, Config};
use subsoil::runtime::BuildStorage;
use topsoil_support::{derive_impl, parameter_types};

type Block = topsoil_system::mocking::MockBlock<Test>;

parameter_types! {
	pub static IndexDeposit: u64 = 1;
}

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: plant_balances,
		Indices: plant_indices,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Nonce = u64;
	type Lookup = Indices;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl Config for Test {
	type AccountIndex = u64;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext: subsoil::io::TestExternalities = t.into();
	// Initialize the block number to 1 for event registration
	ext.execute_with(|| System::set_block_number(1));
	ext
}
