// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

use super::*;
use crate as plant_lottery;

use subsoil::runtime::{BuildStorage, Perbill};
use topsoil_support::{derive_impl, parameter_types, traits::ConstU32};
use topsoil_test_support::TestRandomness;
use topsoil_system::EnsureRoot;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: plant_balances,
		Lottery: plant_lottery,
	}
);

parameter_types! {
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

parameter_types! {
	pub const LotteryPalletId: PalletId = PalletId(*b"py/lotto");
}

impl Config for Test {
	type PalletId = LotteryPalletId;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type Randomness = TestRandomness<Self>;
	type RuntimeEvent = RuntimeEvent;
	type ManagerOrigin = EnsureRoot<u64>;
	type MaxCalls = ConstU32<2>;
	type ValidateCall = Lottery;
	type MaxGenerateRandom = ConstU32<10>;
	type WeightInfo = ();
}

pub type SystemCall = topsoil_system::Call<Test>;
pub type BalancesCall = plant_balances::Call<Test>;

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
