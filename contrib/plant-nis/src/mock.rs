// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test environment for NIS pallet.

use topsoil::{runtime::prelude::*, testing_prelude::*, traits::StorageMapShim};

use crate::{self as plant_nis, *};

pub type Balance = u64;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
#[frame_construct_runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeError,
		RuntimeEvent,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeOrigin,
		RuntimeTask
	)]
	pub struct Test;

	#[runtime::pallet_index(0)]
	pub type System = topsoil_core::system;
	#[runtime::pallet_index(1)]
	pub type Balances = plant_balances<Instance1>;
	#[runtime::pallet_index(2)]
	pub type NisBalances = plant_balances<Instance2>;
	#[runtime::pallet_index(3)]
	pub type Nis = plant_nis;
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<Balance>;
}

impl plant_balances::Config<plant_balances::Instance1> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<1>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type DoneSlashHandler = ();
}

impl plant_balances::Config<plant_balances::Instance2> for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = StorageMapShim<
		plant_balances::Account<Test, plant_balances::Instance2>,
		u64,
		plant_balances::AccountData<u128>,
	>;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

parameter_types! {
	pub IgnoredIssuance: Balance = Balances::total_balance(&0); // Account zero is ignored.
	pub const NisPalletId: PalletId = PalletId(*b"py/nis  ");
	pub static Target: Perquintill = Perquintill::zero();
	pub const MinReceipt: Perquintill = Perquintill::from_percent(1);
	pub const ThawThrottle: (Perquintill, u64) = (Perquintill::from_percent(25), 5);
	pub static MaxIntakeWeight: Weight = Weight::from_parts(2_000_000_000_000, 0);
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl plant_nis::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type PalletId = NisPalletId;
	type Currency = Balances;
	type CurrencyBalance = <Self as plant_balances::Config<plant_balances::Instance1>>::Balance;
	type FundOrigin = topsoil_core::system::EnsureSigned<Self::AccountId>;
	type Deficit = ();
	type IgnoredIssuance = IgnoredIssuance;
	type Counterpart = NisBalances;
	type CounterpartAmount = WithMaximumOf<ConstU128<21_000_000u128>>;
	type Target = Target;
	type QueueCount = ConstU32<3>;
	type MaxQueueLen = ConstU32<3>;
	type FifoQueueLen = ConstU32<1>;
	type BasePeriod = ConstU64<3>;
	type MinBid = ConstU64<2>;
	type IntakePeriod = ConstU64<2>;
	type MaxIntakeWeight = MaxIntakeWeight;
	type MinReceipt = MinReceipt;
	type ThawThrottle = ThawThrottle;
	type RuntimeHoldReason = RuntimeHoldReason;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkSetup = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	plant_balances::GenesisConfig::<Test, plant_balances::Instance1> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup, but without any balances.
#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext_empty() -> subsoil::io::TestExternalities {
	topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
