// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use subsoil::runtime::{traits::Identity, BuildStorage};
use topsoil_core::{derive_impl, parameter_types, traits::WithdrawReasons};

use super::*;
use crate as plant_vesting;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Vesting: plant_vesting,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type AccountData = plant_balances::AccountData<u64>;
	type Block = Block;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
	type ExistentialDeposit = ExistentialDeposit;
}
parameter_types! {
	pub const MinVestedTransfer: u64 = 256 * 2;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
	pub static ExistentialDeposit: u64 = 1;
}
impl Config for Test {
	type BlockNumberToBalance = Identity;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	const MAX_VESTING_SCHEDULES: u32 = 3;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = ();
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
}

pub struct ExtBuilder {
	existential_deposit: u64,
	vesting_genesis_config: Option<Vec<(u64, u64, u64, u64)>>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1, vesting_genesis_config: None }
	}
}

impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}

	pub fn vesting_genesis_config(mut self, config: Vec<(u64, u64, u64, u64)>) -> Self {
		self.vesting_genesis_config = Some(config);
		self
	}

	pub fn build(self) -> subsoil::io::TestExternalities {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
		let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
		plant_balances::GenesisConfig::<Test> {
			balances: vec![
				(1, 10 * self.existential_deposit),
				(2, 20 * self.existential_deposit),
				(3, 30 * self.existential_deposit),
				(4, 40 * self.existential_deposit),
				(12, 10 * self.existential_deposit),
				(13, 9999 * self.existential_deposit),
			],
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let vesting = if let Some(vesting_config) = self.vesting_genesis_config {
			vesting_config
		} else {
			vec![
				(1, 0, 10, 5 * self.existential_deposit),
				(2, 10, 20, 0),
				(12, 10, 20, 5 * self.existential_deposit),
			]
		};

		plant_vesting::GenesisConfig::<Test> { vesting }
			.assimilate_storage(&mut t)
			.unwrap();
		let mut ext = subsoil::io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

parameter_types! {
	static ObservedEventsVesting: usize = 0;
}

pub(crate) fn vesting_events_since_last_call() -> Vec<plant_vesting::Event<Test>> {
	let events = System::read_events_for_pallet::<plant_vesting::Event<Test>>();
	let already_seen = ObservedEventsVesting::get();
	ObservedEventsVesting::set(events.len());
	events.into_iter().skip(already_seen).collect()
}
