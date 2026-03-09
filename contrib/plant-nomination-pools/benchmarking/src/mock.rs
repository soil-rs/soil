// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::VoterBagsListInstance;
use subsoil::runtime::{
	traits::{Convert, IdentityLookup},
	BuildStorage, FixedU128, Perbill,
};
use plant_election_provider::VoteWeight;
use topsoil_core::{
	derive_impl,
	pallet_prelude::*,
	parameter_types,
	traits::{ConstU64, Nothing, VariantCountOf},
	PalletId,
};

type AccountId = u128;
type BlockNumber = u64;
type Balance = u128;

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = plant_balances::AccountData<Balance>;
}

impl plant_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

plant_staking_macros::build! {
	const I_NPOS: subsoil::runtime::curve::PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}
parameter_types! {
	pub const RewardCurve: &'static subsoil::runtime::curve::PiecewiseLinear<'static> = &I_NPOS;
}
#[derive_impl(plant_staking::config_preludes::TestDefaultConfig)]
impl plant_staking::Config for Runtime {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = plant_timestamp::Pallet<Self>;
	type AdminOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type EraPayout = plant_staking::ConvertCurve<RewardCurve>;
	type ElectionProvider =
		plant_election_provider::NoElection<(AccountId, BlockNumber, Staking, (), ())>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = VoterList;
	type TargetList = plant_staking::UseValidatorsMap<Self>;
	type EventListeners = (Pools, DelegatedStaking);
}

parameter_types! {
	pub static BagThresholds: &'static [VoteWeight] = &[10, 20, 30, 40, 50, 60, 1_000, 2_000, 10_000];
}

impl plant_bags_list::Config<VoterBagsListInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type BagThresholds = BagThresholds;
	type ScoreProvider = Staking;
	type Score = VoteWeight;
	type MaxAutoRebagPerBlock = ();
}

pub struct BalanceToU256;
impl Convert<Balance, subsoil::core::U256> for BalanceToU256 {
	fn convert(n: Balance) -> subsoil::core::U256 {
		n.into()
	}
}

pub struct U256ToBalance;
impl Convert<subsoil::core::U256, Balance> for U256ToBalance {
	fn convert(n: subsoil::core::U256) -> Balance {
		n.try_into().unwrap()
	}
}

parameter_types! {
	pub static PostUnbondingPoolsWindow: u32 = 10;
	pub const PoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}

impl plant_nomination_pools::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = Balances;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RewardCounter = FixedU128;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type StakeAdapter =
		plant_nomination_pools::adapter::DelegateStake<Self, Staking, DelegatedStaking>;
	type PostUnbondingPoolsWindow = PostUnbondingPoolsWindow;
	type MaxMetadataLen = ConstU32<256>;
	type MaxUnbonding = ConstU32<8>;
	type PalletId = PoolsPalletId;
	type MaxPointsToBalance = MaxPointsToBalance;
	type AdminOrigin = topsoil_core::system::EnsureRoot<Self::AccountId>;
	type BlockNumberProvider = System;
	type Filter = Nothing;
}

parameter_types! {
	pub const DelegatedStakingPalletId: PalletId = PalletId(*b"py/dlstk");
	pub const SlashRewardFraction: Perbill = Perbill::from_percent(1);
}
impl plant_delegated_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = DelegatedStakingPalletId;
	type Currency = Balances;
	type OnSlash = ();
	type SlashRewardFraction = SlashRewardFraction;
	type RuntimeHoldReason = RuntimeHoldReason;
	type CoreStaking = Staking;
}

impl crate::Config for Runtime {}

type Block = topsoil_core::system::mocking::MockBlock<Runtime>;

topsoil_core::construct_runtime!(
	pub enum Runtime {
		System: topsoil_core::system,
		Timestamp: plant_timestamp,
		Balances: plant_balances,
		Staking: plant_staking,
		VoterList: plant_bags_list::<Instance1>,
		Pools: plant_nomination_pools,
		DelegatedStaking: plant_delegated_staking,
	}
);

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut storage = topsoil_core::system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
	let _ = plant_nomination_pools::GenesisConfig::<Runtime> {
		min_join_bond: 2,
		min_create_bond: 2,
		max_pools: Some(3),
		max_members_per_pool: Some(3),
		max_members: Some(3 * 3),
		global_max_commission: Some(Perbill::from_percent(50)),
	}
	.assimilate_storage(&mut storage);
	subsoil::io::TestExternalities::from(storage)
}
