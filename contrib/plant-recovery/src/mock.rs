// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

use super::*;

use crate as recovery;
use topsoil::{deps::io, testing_prelude::*};

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Recovery: recovery,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u128>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type Balance = u128;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
}

parameter_types! {
	pub static ConfigDepositBase: u64 = 10;
	pub const FriendDepositFactor: u64 = 1;
	pub static RecoveryDeposit: u64 = 10;
	// Large number of friends for benchmarking.
	pub const MaxFriends: u32 = 128;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type RuntimeCall = RuntimeCall;
	type BlockNumberProvider = System;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}

pub type BalancesCall = plant_balances::Call<Test>;
pub type RecoveryCall = super::Call<Test>;

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext: subsoil::io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
