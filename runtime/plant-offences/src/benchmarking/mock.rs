// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mock file for offences benchmarking.

use alloc::{vec, vec::Vec};
use codec::Encode;
use subsoil::runtime::{
	testing::{Header, UintAuthorityId},
	traits::{Saturating, StaticLookup},
	BuildStorage, KeyTypeId, Perbill,
};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use plant_grandpa::TimeSlot as GrandpaTimeSlot;
use plant_session::historical as pallet_session_historical;
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};
use topsoil_system as system;

const SEED: u32 = 0;
const MAX_BENCH_NOMINATORS: u32 = 100;

type AccountId = u64;
type IdentificationTuple = plant_session::historical::IdentificationTuple<Test>;

type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
type UncheckedExtrinsic =
	subsoil::runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, u64, ()>;

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u64>;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type ExistentialDeposit = ConstU64<10>;
	type AccountStore = System;
}

impl plant_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

impl plant_session::historical::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = ();
	type FullIdentificationOf = plant_staking::UnitIdentificationOf<Self>;
}

subsoil::impl_opaque_keys! {
	pub struct SessionKeys {
		pub foo: subsoil::runtime::testing::UintAuthorityId,
	}
}

pub struct TestSessionHandler;
impl plant_session::SessionHandler<AccountId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [KeyTypeId] = &[KeyTypeId([100u8, 117u8, 109u8, 121u8])];

	fn on_genesis_session<Ks: subsoil::runtime::traits::OpaqueKeys>(
		_validators: &[(AccountId, Ks)],
	) {
	}

	fn on_new_session<Ks: subsoil::runtime::traits::OpaqueKeys>(
		_: bool,
		_: &[(AccountId, Ks)],
		_: &[(AccountId, Ks)],
	) {
	}

	fn on_disabled(_: u32) {}
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

impl plant_session::Config for Test {
	type SessionManager = plant_session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = SessionKeys;
	type ShouldEndSession = plant_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = plant_session::PeriodicSessions<Period, Offset>;
	type SessionHandler = TestSessionHandler;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = subsoil::runtime::traits::ConvertInto;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ();
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
	pub static ElectionsBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
	pub const Sort: bool = true;
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<AccountId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinnersPerPage = ConstU32<100>;
	type MaxBackersPerWinner = ConstU32<100>;
	type Sort = Sort;
	type Bounds = ElectionsBounds;
}

#[derive_impl(plant_staking::config_preludes::TestDefaultConfig)]
impl plant_staking::Config for Test {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = <Self as plant_balances::Config>::Balance;
	type UnixTime = plant_timestamp::Pallet<Self>;
	type AdminOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type EraPayout = plant_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = plant_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = plant_staking::UseValidatorsMap<Self>;
}

impl plant_im_online::Config for Test {
	type AuthorityId = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = plant_session::PeriodicSessions<Period, Offset>;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ();
	type WeightInfo = ();
	type MaxKeys = ConstU32<10_000>;
	type MaxPeerInHeartbeats = ConstU32<10_000>;
}

impl plant_offences::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = IdentificationTuple;
	type OnOffenceHandler = Staking;
}

impl<T> topsoil_system::offchain::CreateTransactionBase<T> for Test
where
	RuntimeCall: From<T>,
{
	type Extrinsic = UncheckedExtrinsic;
	type RuntimeCall = RuntimeCall;
}

impl<T> topsoil_system::offchain::CreateBare<T> for Test
where
	RuntimeCall: From<T>,
{
	fn create_bare(call: Self::RuntimeCall) -> Self::Extrinsic {
		UncheckedExtrinsic::new_bare(call)
	}
}

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: system::{Pallet, Call, Event<T>},
		Balances: plant_balances,
		Staking: plant_staking,
		Session: plant_session,
		ImOnline: plant_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
		Offences: plant_offences::{Pallet, Storage, Event},
		Historical: pallet_session_historical::{Pallet, Event<T>},
	}
);

struct Offender {
	controller: AccountId,
	#[allow(dead_code)]
	stash: AccountId,
	#[allow(dead_code)]
	nominator_stashes: Vec<AccountId>,
}

fn generate_session_keys_and_proof(owner: AccountId) -> (SessionKeys, Vec<u8>) {
	let keys = SessionKeys::generate(&owner.encode(), None);
	(keys.keys, keys.proof.encode())
}

fn bond_amount() -> <Test as plant_staking::Config>::CurrencyBalance {
	plant_staking::asset::existential_deposit::<Test>().saturating_mul(10_000u32.into())
}

fn create_offender(n: u32, nominators: u32) -> Result<Offender, &'static str> {
	let stash: AccountId = account("stash", n, SEED);
	let stash_lookup = <Test as topsoil_system::Config>::Lookup::unlookup(stash.clone());
	let reward_destination = plant_staking::RewardDestination::Staked;
	let amount = bond_amount();
	let free_amount = amount.saturating_mul(2u32.into());
	plant_staking::asset::set_stakeable_balance::<Test>(&stash, free_amount);
	plant_staking::Pallet::<Test>::bond(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		amount,
		reward_destination.clone(),
	)
	.map_err(|_| "failed to bond stash")?;

	let validator_prefs = plant_staking::ValidatorPrefs {
		commission: Perbill::from_percent(50),
		..Default::default()
	};
	plant_staking::Pallet::<Test>::validate(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		validator_prefs,
	)
	.map_err(|_| "failed to validate")?;

	let (keys, proof) = generate_session_keys_and_proof(stash.clone());
	plant_session::Pallet::<Test>::ensure_can_pay_key_deposit(&stash).map_err(|_| "key deposit")?;
	plant_session::Pallet::<Test>::set_keys(
		topsoil_system::RawOrigin::Signed(stash.clone()).into(),
		keys,
		proof,
	)
	.map_err(|_| "failed to set keys")?;

	let mut individual_exposures = vec![];
	let mut nominator_stashes = vec![];
	for i in 0..nominators {
		let nominator_stash: AccountId =
			account("nominator stash", n * MAX_BENCH_NOMINATORS + i, SEED);
		plant_staking::asset::set_stakeable_balance::<Test>(&nominator_stash, free_amount);
		plant_staking::Pallet::<Test>::bond(
			topsoil_system::RawOrigin::Signed(nominator_stash.clone()).into(),
			amount,
			reward_destination.clone(),
		)
		.map_err(|_| "failed to bond nominator")?;
		let selected_validators = vec![stash_lookup.clone()];
		plant_staking::Pallet::<Test>::nominate(
			topsoil_system::RawOrigin::Signed(nominator_stash.clone()).into(),
			selected_validators,
		)
		.map_err(|_| "failed to nominate")?;
		individual_exposures.push(plant_staking::IndividualExposure {
			who: nominator_stash.clone(),
			value: amount,
		});
		nominator_stashes.push(nominator_stash);
	}

	let exposure = plant_staking::Exposure {
		total: amount.saturating_mul((nominators + 1).into()),
		own: amount,
		others: individual_exposures,
	};
	plant_staking::Pallet::<Test>::add_era_stakers(0, stash.clone(), exposure);

	Ok(Offender { controller: stash.clone(), stash, nominator_stashes })
}

fn make_offenders(num_offenders: u32, num_nominators: u32) -> Result<Vec<IdentificationTuple>, &'static str> {
	let mut offenders = vec![];
	for i in 0..num_offenders {
		let offender = create_offender(i + 1, num_nominators)?;
		plant_session::Validators::<Test>::mutate(|validators| validators.push(offender.controller.clone()));
		offenders.push(offender);
	}

	let id_tuples = offenders
		.iter()
		.map(|offender| offender.controller.clone())
		.map(|validator_id| {
			<Test as plant_session::historical::Config>::FullIdentificationOf::convert(validator_id.clone())
				.map(|full_id| (validator_id, full_id))
				.expect("historical identification should exist")
		})
		.collect::<Vec<_>>();

	if plant_staking::ActiveEra::<Test>::get().is_none() {
		plant_staking::ActiveEra::<Test>::put(plant_staking::ActiveEraInfo {
			index: 0,
			start: Some(0),
		});
	}

	Ok(id_tuples)
}

fn assert_all_slashes_applied(offender_count: usize) {
	assert_eq!(topsoil_system::Pallet::<Test>::read_events_for_pallet::<plant_balances::Event<Test>>().len(), 3);
	assert_eq!(
		topsoil_system::Pallet::<Test>::read_events_for_pallet::<plant_staking::Event<Test>>().len(),
		1 * (offender_count + 1) + 1
	);
	assert_eq!(topsoil_system::Pallet::<Test>::read_events_for_pallet::<plant_offences::Event>().len(), 1);
	assert_eq!(topsoil_system::Pallet::<Test>::read_events_for_pallet::<topsoil_system::Event<Test>>().len(), 1);
}

impl super::Config for Test {
	type MaxNominators = plant_staking::MaxNominationsOf<Self>;
	type BabeBenchmarkOffence = plant_babe::EquivocationOffence<Self::IdentificationTuple>;
	type GrandpaBenchmarkOffence = plant_grandpa::EquivocationOffence<Self::IdentificationTuple>;

	fn setup_babe_benchmark(
		n: u32,
	) -> Result<(Vec<Self::AccountId>, Self::BabeBenchmarkOffence), &'static str> {
		let reporters = vec![account("reporter", 1, SEED)];
		plant_staking::Pallet::<Self>::set_slash_reward_fraction(Perbill::one());
		let mut offenders = make_offenders(1, n)?;
		let validator_set_count = plant_session::Pallet::<Self>::validators().len() as u32;
		let offence = plant_babe::EquivocationOffence {
			slot: 0u64.into(),
			session_index: 0,
			validator_set_count,
			offender: offenders.pop().ok_or("missing offender")?,
		};
		assert_eq!(topsoil_system::Pallet::<Self>::event_count(), 0);
		Ok((reporters, offence))
	}

	fn setup_grandpa_benchmark(
		n: u32,
	) -> Result<(Vec<Self::AccountId>, Self::GrandpaBenchmarkOffence), &'static str> {
		let reporters = vec![account("reporter", 1, SEED)];
		plant_staking::Pallet::<Self>::set_slash_reward_fraction(Perbill::one());
		let mut offenders = make_offenders(1, n)?;
		let validator_set_count = plant_session::Pallet::<Self>::validators().len() as u32;
		let offence = plant_grandpa::EquivocationOffence {
			time_slot: GrandpaTimeSlot { set_id: 0, round: 0 },
			session_index: 0,
			validator_set_count,
			offender: offenders.pop().ok_or("missing offender")?,
		};
		assert_eq!(topsoil_system::Pallet::<Self>::event_count(), 0);
		Ok((reporters, offence))
	}

	#[cfg(test)]
	fn assert_all_slashes_applied(offender_count: usize) {
		assert_all_slashes_applied(offender_count);
	}
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	subsoil::tracing::try_init_simple();
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	subsoil::io::TestExternalities::new(t)
}
