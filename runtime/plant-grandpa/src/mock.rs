// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

#![cfg(test)]

use crate::{self as plant_grandpa, AuthorityId, AuthorityList, Config, ConsensusLog};
use codec::Encode;
use finality_grandpa;
use subsoil::staking::{EraIndex, SessionIndex};
use subsoil::consensus::grandpa::{RoundNumber, SetId, GRANDPA_ENGINE_ID};
use subsoil::core::{ConstBool, H256};
use subsoil::keyring::Ed25519Keyring;
use subsoil::runtime::{
	curve::PiecewiseLinear,
	testing::{TestXt, UintAuthorityId},
	traits::OpaqueKeys,
	BuildStorage, DigestItem, Perbill,
};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use plant_session::historical as pallet_session_historical;
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, OnFinalize, OnInitialize},
};

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Authorship: plant_authorship,
		Timestamp: plant_timestamp,
		Balances: plant_balances,
		Staking: plant_staking,
		Session: plant_session,
		Grandpa: plant_grandpa,
		Offences: plant_offences,
		Historical: pallet_session_historical,
	}
);

subsoil::impl_opaque_keys! {
	pub struct TestSessionKeys {
		pub grandpa_authority: super::Pallet<Test>,
	}
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<Balance>;
}

impl<C> topsoil_system::offchain::CreateTransactionBase<C> for Test
where
	RuntimeCall: From<C>,
{
	type RuntimeCall = RuntimeCall;
	type Extrinsic = TestXt<RuntimeCall, ()>;
}

impl<C> topsoil_system::offchain::CreateBare<C> for Test
where
	RuntimeCall: From<C>,
{
	fn create_bare(call: Self::RuntimeCall) -> Self::Extrinsic {
		TestXt::new_bare(call)
	}
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

/// Custom `SessionHandler` since we use `TestSessionKeys` as `Keys`.
impl plant_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = u64;
	type ValidatorIdOf = subsoil::runtime::traits::ConvertInto;
	type ShouldEndSession = plant_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type NextSessionRotation = plant_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
	type SessionManager = plant_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <TestSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = TestSessionKeys;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ();
}

impl plant_session::historical::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = ();
	type FullIdentificationOf = plant_staking::UnitIdentificationOf<Self>;
}

impl plant_authorship::Config for Test {
	type FindAuthor = ();
	type EventHandler = ();
}

type Balance = u128;
#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
}

impl plant_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<3>;
	type WeightInfo = ();
}

plant_staking_macros::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000u64,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	pub const SessionsPerEra: SessionIndex = 3;
	pub const BondingDuration: EraIndex = 3;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub static ElectionsBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default().build();
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<u64, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinnersPerPage = ConstU32<100>;
	type MaxBackersPerWinner = ConstU32<100>;
	type Sort = ConstBool<true>;
	type Bounds = ElectionsBoundsOnChain;
}

#[derive_impl(plant_staking::config_preludes::TestDefaultConfig)]
impl plant_staking::Config for Test {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = <Self as plant_balances::Config>::Balance;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type AdminOrigin = topsoil_system::EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type UnixTime = plant_timestamp::Pallet<Test>;
	type EraPayout = plant_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = plant_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = plant_staking::UseValidatorsMap<Self>;
	type NominationsQuota = plant_staking::FixedNominationsQuota<16>;
}

impl plant_offences::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = plant_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

parameter_types! {
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * Period::get();
	pub const MaxSetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxAuthorities = ConstU32<100>;
	type MaxNominators = ConstU32<1000>;
	type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
	type KeyOwnerProof = subsoil::session::MembershipProof;
	type EquivocationReportSystem =
		super::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

pub fn grandpa_log(log: ConsensusLog<u64>) -> DigestItem {
	DigestItem::Consensus(GRANDPA_ENGINE_ID, log.encode())
}

pub fn to_authorities(vec: Vec<(u64, u64)>) -> AuthorityList {
	vec.into_iter()
		.map(|(id, weight)| (UintAuthorityId(id).to_public_key::<AuthorityId>(), weight))
		.collect()
}

pub fn extract_keyring(id: &AuthorityId) -> Ed25519Keyring {
	let mut raw_public = [0; 32];
	raw_public.copy_from_slice(id.as_ref());
	Ed25519Keyring::from_raw_public(raw_public).unwrap()
}

pub fn new_test_ext(vec: Vec<(u64, u64)>) -> subsoil::io::TestExternalities {
	new_test_ext_raw_authorities(to_authorities(vec))
}

pub fn new_test_ext_raw_authorities(authorities: AuthorityList) -> subsoil::io::TestExternalities {
	subsoil::tracing::try_init_simple();
	let mut t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let balances: Vec<_> = (0..authorities.len()).map(|i| (i as u64, 10_000_000)).collect();

	plant_balances::GenesisConfig::<Test> { balances, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	// stashes are the index.
	let session_keys: Vec<_> = authorities
		.iter()
		.enumerate()
		.map(|(i, (k, _))| {
			(
				i as u64,
				i as u64,
				TestSessionKeys { grandpa_authority: AuthorityId::from(k.clone()) },
			)
		})
		.collect();

	// NOTE: this will initialize the grandpa authorities
	// through OneSessionHandler::on_genesis_session
	plant_session::GenesisConfig::<Test> { keys: session_keys, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	// controllers are the same as stash
	let stakers: Vec<_> = (0..authorities.len())
		.map(|i| (i as u64, i as u64, 10_000, plant_staking::StakerStatus::<u64>::Validator))
		.collect();

	let staking_config = plant_staking::GenesisConfig::<Test> {
		stakers,
		validator_count: 8,
		force_era: plant_staking::Forcing::ForceNew,
		minimum_validator_count: 0,
		invulnerables: vec![],
		..Default::default()
	};

	staking_config.assimilate_storage(&mut t).unwrap();

	t.into()
}

pub fn start_session(session_index: SessionIndex) {
	for i in Session::current_index()..session_index {
		System::on_finalize(System::block_number());
		Session::on_finalize(System::block_number());
		Staking::on_finalize(System::block_number());
		Grandpa::on_finalize(System::block_number());

		let parent_hash = if System::block_number() > 1 {
			let hdr = System::finalize();
			hdr.hash()
		} else {
			System::parent_hash()
		};

		System::reset_events();
		System::initialize(&(i as u64 + 1), &parent_hash, &Default::default());
		System::set_block_number((i + 1).into());
		Timestamp::set_timestamp(System::block_number() * 6000);

		System::on_initialize(System::block_number());
		Session::on_initialize(System::block_number());
		Staking::on_initialize(System::block_number());
		Grandpa::on_initialize(System::block_number());
	}

	assert_eq!(Session::current_index(), session_index);
}

pub fn start_era(era_index: EraIndex) {
	start_session((era_index * 3).into());
	assert_eq!(plant_staking::CurrentEra::<Test>::get(), Some(era_index));
}

pub fn initialize_block(number: u64, parent_hash: H256) {
	System::reset_events();
	System::initialize(&number, &parent_hash, &Default::default());
}

pub fn generate_equivocation_proof(
	set_id: SetId,
	vote1: (RoundNumber, H256, u64, &Ed25519Keyring),
	vote2: (RoundNumber, H256, u64, &Ed25519Keyring),
) -> subsoil::consensus::grandpa::EquivocationProof<H256, u64> {
	let signed_prevote = |round, hash, number, keyring: &Ed25519Keyring| {
		let prevote = finality_grandpa::Prevote { target_hash: hash, target_number: number };

		let prevote_msg = finality_grandpa::Message::Prevote(prevote.clone());
		let payload = subsoil::consensus::grandpa::localized_payload(round, set_id, &prevote_msg);
		let signed = keyring.sign(&payload).into();
		(prevote, signed)
	};

	let (prevote1, signed1) = signed_prevote(vote1.0, vote1.1, vote1.2, vote1.3);
	let (prevote2, signed2) = signed_prevote(vote2.0, vote2.1, vote2.2, vote2.3);

	subsoil::consensus::grandpa::EquivocationProof::new(
		set_id,
		subsoil::consensus::grandpa::Equivocation::Prevote(finality_grandpa::Equivocation {
			round_number: vote1.0,
			identity: vote1.3.public().into(),
			first: (prevote1, signed1),
			second: (prevote2, signed2),
		}),
	)
}
