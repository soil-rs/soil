// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests and test utilities for transaction pause pallet.

#![cfg(test)]

use super::*;
use crate as plant_tx_pause;
use topsoil::testing_prelude::*;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type BaseCallFilter = InsideBoth<Everything, TxPause>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl plant_utility::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

/// Mocked proxies to check that tx-pause also works with the proxy pallet.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	DecodeWithMemTracking,
	Debug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	JustTransfer,
	JustUtility,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::JustTransfer => {
				matches!(
					c,
					RuntimeCall::Balances(plant_balances::Call::transfer_allow_death { .. })
				)
			},
			ProxyType::JustUtility => matches!(c, RuntimeCall::Utility { .. }),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		self == &ProxyType::Any || self == o
	}
}

impl plant_proxy::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ConstU64<1>;
	type ProxyDepositFactor = ConstU64<1>;
	type MaxProxies = ConstU32<4>;
	type WeightInfo = ();
	type CallHasher = BlakeTwo256;
	type MaxPending = ConstU32<2>;
	type AnnouncementDepositBase = ConstU64<1>;
	type AnnouncementDepositFactor = ConstU64<1>;
	type BlockNumberProvider = topsoil_core::system::Pallet<Test>;
}

parameter_types! {
	pub const MaxNameLen: u32 = 50;
}

ord_parameter_types! {
	pub const PauseOrigin: u64 = 1;
	pub const UnpauseOrigin: u64 = 2;
}

/// Calls that are never allowed to be paused.
pub struct WhitelistedCalls;
impl Contains<RuntimeCallNameOf<Test>> for WhitelistedCalls {
	fn contains(full_name: &RuntimeCallNameOf<Test>) -> bool {
		match (full_name.0.as_slice(), full_name.1.as_slice()) {
			(b"Balances", b"transfer_keep_alive") => true,
			_ => false,
		}
	}
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PauseOrigin = EnsureSignedBy<PauseOrigin, Self::AccountId>;
	type UnpauseOrigin = EnsureSignedBy<UnpauseOrigin, Self::AccountId>;
	type WhitelistedCalls = WhitelistedCalls;
	type MaxNameLen = MaxNameLen;
	type WeightInfo = ();
}

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Utility: plant_utility,
		Proxy: plant_proxy,
		TxPause: plant_tx_pause,
	}
);

pub fn new_test_ext() -> TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	plant_balances::GenesisConfig::<Test> {
		// The 0 account is NOT a special origin. The rest may be:
		balances: vec![(0, 1234), (1, 5678), (2, 5678), (3, 5678), (4, 5678)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	plant_tx_pause::GenesisConfig::<Test> { paused: vec![] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn next_block() {
	TxPause::on_finalize(System::block_number());
	Balances::on_finalize(System::block_number());
	System::on_finalize(System::block_number());
	System::set_block_number(System::block_number() + 1);
	System::on_initialize(System::block_number());
	Balances::on_initialize(System::block_number());
	TxPause::on_initialize(System::block_number());
}

pub fn run_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
}
