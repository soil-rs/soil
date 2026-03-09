// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests mock for `topsoil-assets-freezer`.

use crate as plant_assets_freezer;
pub use crate::*;
use codec::{Compact, Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use topsoil::testing_prelude::*;

pub type AccountId = u64;
pub type Balance = u64;
pub type AssetId = u32;
type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Assets: plant_assets,
		AssetsFreezer: plant_assets_freezer,
		Balances: plant_balances,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type Hash = H256;
	type RuntimeCall = RuntimeCall;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = plant_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl plant_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

impl plant_assets::Config for Test {
	type AssetId = AssetId;
	type AssetIdParameter = Compact<AssetId>;
	type ReserveData = ();
	type AssetDeposit = ConstU64<1>;
	type Balance = Balance;
	type AssetAccountDeposit = ConstU64<1>;
	type MetadataDepositBase = ();
	type MetadataDepositPerByte = ();
	type ApprovalDeposit = ();
	type CreateOrigin = AsEnsureOriginWithArg<topsoil_core::system::EnsureSigned<u64>>;
	type ForceOrigin = topsoil_core::system::EnsureRoot<u64>;
	type StringLimit = ConstU32<32>;
	type Extra = ();
	type RemoveItemsLimit = ConstU32<10>;
	type CallbackHandle = ();
	type Currency = Balances;
	type Holder = ();
	type Freezer = AssetsFreezer;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

#[derive(
	Decode,
	DecodeWithMemTracking,
	Encode,
	MaxEncodedLen,
	PartialEq,
	Eq,
	Ord,
	PartialOrd,
	TypeInfo,
	Debug,
	Clone,
	Copy,
)]
pub enum DummyFreezeReason {
	Governance,
	Staking,
	Other,
}

impl VariantCount for DummyFreezeReason {
	// Intentionally set below the actual count of variants, to allow testing for `can_freeze`
	const VARIANT_COUNT: u32 = 2;
}

impl Config for Test {
	type RuntimeFreezeReason = DummyFreezeReason;
	type RuntimeEvent = RuntimeEvent;
}

pub fn new_test_ext(execute: impl FnOnce()) -> TestExternalities {
	let t = RuntimeGenesisConfig {
		assets: plant_assets::GenesisConfig {
			assets: vec![(1, 0, true, 1)],
			metadata: vec![],
			accounts: vec![(1, 1, 100)],
			next_asset_id: None,
			reserves: vec![],
		},
		system: Default::default(),
		balances: Default::default(),
	}
	.build_storage()
	.unwrap();
	let mut ext: TestExternalities = t.into();
	ext.execute_with(|| {
		System::set_block_number(1);
		execute();
		#[cfg(feature = "try-runtime")]
		assert_ok!(AssetsFreezer::do_try_state());
	});

	ext
}
