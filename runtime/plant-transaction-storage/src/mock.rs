// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for transaction-storage pallet.

use crate::{
	self as plant_transaction_storage, TransactionStorageProof, DEFAULT_MAX_BLOCK_TRANSACTIONS,
	DEFAULT_MAX_TRANSACTION_SIZE,
};
use subsoil::runtime::{traits::IdentityLookup, BuildStorage};
use topsoil_core::{derive_impl, traits::ConstU32};

pub type Block = topsoil_core::system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		TransactionStorage: plant_transaction_storage,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
}

impl plant_transaction_storage::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type FeeDestination = ();
	type WeightInfo = ();
	type MaxBlockTransactions = ConstU32<{ DEFAULT_MAX_BLOCK_TRANSACTIONS }>;
	type MaxTransactionSize = ConstU32<{ DEFAULT_MAX_TRANSACTION_SIZE }>;
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = RuntimeGenesisConfig {
		system: Default::default(),
		balances: plant_balances::GenesisConfig::<Test> {
			balances: vec![(1, 1000000000), (2, 100), (3, 100), (4, 100)],
			..Default::default()
		},
		transaction_storage: plant_transaction_storage::GenesisConfig::<Test> {
			retention_period: 10,
			byte_fee: 2,
			entry_fee: 200,
		},
	}
	.build_storage()
	.unwrap();
	t.into()
}

pub fn run_to_block(n: u64, f: impl Fn() -> Option<TransactionStorageProof> + 'static) {
	System::run_to_block_with::<AllPalletsWithSystem>(
		n,
		topsoil_core::system::RunToBlockHooks::default().before_finalize(|_| {
			if let Some(proof) = f() {
				TransactionStorage::check_proof(RuntimeOrigin::none(), proof).unwrap();
			}
		}),
	);
}
