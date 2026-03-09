// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for Uniques pallet.

use super::*;
use crate as plant_uniques;

use subsoil::runtime::BuildStorage;
use topsoil_core::{
	construct_runtime, derive_impl,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64},
};

type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Balances: plant_balances,
		Uniques: plant_uniques,
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

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<topsoil_core::system::EnsureSigned<u64>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<u64>;
	type Locker = ();
	type CollectionDeposit = ConstU64<2>;
	type ItemDeposit = ConstU64<1>;
	type MetadataDepositBase = ConstU64<1>;
	type AttributeDepositBase = ConstU64<1>;
	type DepositPerByte = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
