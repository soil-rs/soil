// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::{self as delegated_staking, types::AgentLedgerOuter};
use topsoil_support::{
	assert_ok, derive_impl,
	pallet_prelude::*,
	parameter_types,
	traits::{ConstU64, Currency, VariantCountOf},
	PalletId,
};

use subsoil::runtime::{traits::IdentityLookup, BuildStorage, Perbill};

use subsoil::staking::{Agent, Stake, StakingInterface};
use subsoil::core::{ConstBool, U256};
use subsoil::runtime::traits::Convert;
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use plant_staking::{ActiveEra, ActiveEraInfo, CurrentEra};
use topsoil_support::dispatch::RawOrigin;

pub type T = Runtime;
type Block = topsoil_system::mocking::MockBlock<Runtime>;
pub type AccountId = u128;

pub const GENESIS_VALIDATOR: AccountId = 1;
pub const GENESIS_NOMINATOR_ONE: AccountId = 101;
pub const GENESIS_NOMINATOR_TWO: AccountId = 102;

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type Block = Block;
	type AccountData = plant_balances::AccountData<Balance>;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
}

impl plant_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

pub type Balance = u128;

parameter_types! {
	pub static ExistentialDeposit: Balance = 1;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
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
	pub static ElectionsBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default().build();
}
pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<Balance, subsoil::runtime::Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinnersPerPage = ConstU32<100>;
	type MaxBackersPerWinner = ConstU32<100>;
	type Sort = ConstBool<true>;
	type Bounds = ElectionsBoundsOnChain;
}

#[derive_impl(plant_staking::config_preludes::TestDefaultConfig)]
impl plant_staking::Config for Runtime {
	type OldCurrency = Balances;
	type Currency = Balances;
	type UnixTime = plant_timestamp::Pallet<Self>;
	type AdminOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type EraPayout = plant_staking::ConvertCurve<RewardCurve>;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = plant_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = plant_staking::UseValidatorsMap<Self>;
	type EventListeners = (Pools, DelegatedStaking);
	type Filter = plant_nomination_pools::AllPoolMembers<Self>;
}

parameter_types! {
	pub const DelegatedStakingPalletId: PalletId = PalletId(*b"py/dlstk");
	pub const SlashRewardFraction: Perbill = Perbill::from_percent(10);
}
impl delegated_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = DelegatedStakingPalletId;
	type Currency = Balances;
	type OnSlash = ();
	type SlashRewardFraction = SlashRewardFraction;
	type RuntimeHoldReason = RuntimeHoldReason;
	type CoreStaking = Staking;
}

pub struct BalanceToU256;
impl Convert<Balance, U256> for BalanceToU256 {
	fn convert(n: Balance) -> U256 {
		n.into()
	}
}
pub struct U256ToBalance;
impl Convert<U256, Balance> for U256ToBalance {
	fn convert(n: U256) -> Balance {
		n.try_into().unwrap()
	}
}

parameter_types! {
	pub static MaxUnbonding: u32 = 8;
	pub const PoolsPalletId: PalletId = PalletId(*b"py/nopls");
}
impl plant_nomination_pools::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = Balances;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RewardCounter = subsoil::runtime::FixedU128;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type PostUnbondingPoolsWindow = ConstU32<2>;
	type PalletId = PoolsPalletId;
	type MaxMetadataLen = ConstU32<256>;
	type MaxUnbonding = MaxUnbonding;
	type MaxPointsToBalance = topsoil_support::traits::ConstU8<10>;
	type StakeAdapter =
		plant_nomination_pools::adapter::DelegateStake<Self, Staking, DelegatedStaking>;
	type AdminOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type BlockNumberProvider = System;
	type Filter = plant_staking::AllStakers<Runtime>;
}

topsoil_support::construct_runtime!(
	pub enum Runtime {
		System: topsoil_system,
		Timestamp: plant_timestamp,
		Balances: plant_balances,
		Staking: plant_staking,
		Pools: plant_nomination_pools,
		DelegatedStaking: delegated_staking,
	}
);

#[derive(Default)]
pub struct ExtBuilder {}

impl ExtBuilder {
	fn build(self) -> subsoil::io::TestExternalities {
		subsoil::tracing::try_init_simple();
		let mut storage =
			topsoil_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

		let _ = plant_balances::GenesisConfig::<T> {
			balances: vec![
				(GENESIS_VALIDATOR, 10000),
				(GENESIS_NOMINATOR_ONE, 1000),
				(GENESIS_NOMINATOR_TWO, 2000),
			],
			..Default::default()
		}
		.assimilate_storage(&mut storage);

		let stakers = vec![
			(
				GENESIS_VALIDATOR,
				GENESIS_VALIDATOR,
				1000,
				subsoil::staking::StakerStatus::<AccountId>::Validator,
			),
			(
				GENESIS_NOMINATOR_ONE,
				GENESIS_NOMINATOR_ONE,
				100,
				subsoil::staking::StakerStatus::<AccountId>::Nominator(vec![1]),
			),
			(
				GENESIS_NOMINATOR_TWO,
				GENESIS_NOMINATOR_TWO,
				200,
				subsoil::staking::StakerStatus::<AccountId>::Nominator(vec![1]),
			),
		];

		let _ = plant_staking::GenesisConfig::<T> {
			stakers: stakers.clone(),
			// ideal validator count
			validator_count: 2,
			minimum_validator_count: 1,
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			min_nominator_bond: ExistentialDeposit::get(),
			min_validator_bond: ExistentialDeposit::get(),
			..Default::default()
		}
		.assimilate_storage(&mut storage);

		let mut ext = subsoil::io::TestExternalities::from(storage);

		ext.execute_with(|| {
			// for events to be deposited.
			topsoil_system::Pallet::<Runtime>::set_block_number(1);
			// set era for staking.
			start_era(0);
		});

		ext
	}
	pub fn build_and_execute(self, test: impl FnOnce()) {
		subsoil::tracing::try_init_simple();
		let mut ext = self.build();
		ext.execute_with(test);
		ext.execute_with(|| {
			#[cfg(feature = "try-runtime")]
			<AllPalletsWithSystem as topsoil_support::traits::TryState<u64>>::try_state(
				topsoil_system::Pallet::<Runtime>::block_number(),
				topsoil_support::traits::TryStateSelect::All,
			)
			.unwrap();
			#[cfg(not(feature = "try-runtime"))]
			DelegatedStaking::do_try_state().unwrap();
		});
	}
}

/// fund and return who.
pub(crate) fn fund(who: &AccountId, amount: Balance) {
	let _ = Balances::deposit_creating(who, amount);
}

/// Sets up delegation for passed delegators, returns total delegated amount.
///
/// `delegate_amount` is incremented by the amount `increment` starting with `base_delegate_amount`
/// from lower index to higher index of delegators.
pub(crate) fn setup_delegation_stake(
	agent: AccountId,
	reward_acc: AccountId,
	delegators: Vec<AccountId>,
	base_delegate_amount: Balance,
	increment: Balance,
) -> Balance {
	fund(&agent, 100);
	assert_ok!(DelegatedStaking::register_agent(RawOrigin::Signed(agent).into(), reward_acc));
	let mut delegated_amount: Balance = 0;
	for (index, delegator) in delegators.iter().enumerate() {
		let amount_to_delegate = base_delegate_amount + increment * index as Balance;
		delegated_amount += amount_to_delegate;

		fund(delegator, amount_to_delegate + ExistentialDeposit::get());
		assert_ok!(DelegatedStaking::delegate_to_agent(
			RawOrigin::Signed(*delegator).into(),
			agent,
			amount_to_delegate
		));
	}

	// sanity checks
	assert_eq!(DelegatedStaking::stakeable_balance(Agent::from(agent)), delegated_amount);
	assert_eq!(AgentLedgerOuter::<T>::get(&agent).unwrap().available_to_bond(), 0);

	delegated_amount
}

pub(crate) fn start_era(era: subsoil::staking::EraIndex) {
	CurrentEra::<T>::set(Some(era));
	ActiveEra::<T>::set(Some(ActiveEraInfo { index: era, start: None }));
}

pub(crate) fn eq_stake(who: AccountId, total: Balance, active: Balance) -> bool {
	Staking::stake(&who).unwrap() == Stake { total, active }
		&& get_agent_ledger(&who).ledger.stakeable_balance() == total
}

pub(crate) fn get_agent_ledger(agent: &AccountId) -> AgentLedgerOuter<T> {
	AgentLedgerOuter::<T>::get(agent).expect("delegate should exist")
}

parameter_types! {
	static ObservedEventsDelegatedStaking: usize = 0;
	static ObservedEventsPools: usize = 0;
}

pub(crate) fn pool_events_since_last_call() -> Vec<plant_nomination_pools::Event<Runtime>> {
	let events = System::read_events_for_pallet::<plant_nomination_pools::Event<Runtime>>();
	let already_seen = ObservedEventsPools::get();
	ObservedEventsPools::set(events.len());
	events.into_iter().skip(already_seen).collect()
}

pub(crate) fn events_since_last_call() -> Vec<crate::Event<Runtime>> {
	let events = System::read_events_for_pallet::<crate::Event<Runtime>>();
	let already_seen = ObservedEventsDelegatedStaking::get();
	ObservedEventsDelegatedStaking::set(events.len());
	events.into_iter().skip(already_seen).collect()
}
