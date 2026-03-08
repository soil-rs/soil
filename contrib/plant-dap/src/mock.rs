// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test mock for the DAP pallet.

use crate::{self as plant_dap, Config};
use subsoil::runtime::BuildStorage;
use topsoil_support::{
	derive_impl, parameter_types, subsoil::runtime::traits::AccountIdConversion, PalletId,
};

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test {
		System: topsoil_system,
		Balances: plant_balances,
		Dap: plant_dap,
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
	type ExistentialDeposit = ExistentialDeposit;
}

parameter_types! {
	pub const DapPalletId: PalletId = PalletId(*b"dap/buff");
	pub const ExistentialDeposit: u64 = 10;
}

impl Config for Test {
	type Currency = Balances;
	type PalletId = DapPalletId;
}

pub fn new_test_ext(fund_buffer: bool) -> subsoil::io::TestExternalities {
	let mut balances = vec![(1, 100), (2, 200), (3, 300)];

	if fund_buffer {
		let buffer: u64 = DapPalletId::get().into_account_truncating();
		balances.push((buffer, ExistentialDeposit::get()));
	}

	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_balances::GenesisConfig::<Test> { balances, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();
	t.into()
}
