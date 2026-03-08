// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

use plant_babe::{self, Config, CurrentSlot};
use codec::Encode;
use subsoil::staking::{EraIndex, SessionIndex};
use subsoil::consensus::babe::{AuthorityId, AuthorityPair, Randomness, Slot, VrfSignature};
use subsoil::core::{
	crypto::{Pair, VrfSecret},
	ConstBool, U256,
};
use subsoil::runtime::{
	curve::PiecewiseLinear,
	testing::{Digest, DigestItem, Header, TestXt},
	traits::{Header as _, OpaqueKeys},
	BuildStorage, DispatchError, Perbill,
};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use plant_session::historical as pallet_session_historical;
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, OnInitialize},
};

type DummyValidatorId = u64;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Authorship: plant_authorship,
		Balances: plant_balances,
		Historical: pallet_session_historical,
		Offences: plant_offences,
		Babe: plant_babe,
		Staking: plant_staking,
		Session: plant_session,
		Timestamp: plant_timestamp,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type AccountData = plant_balances::AccountData<u128>;
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

subsoil::impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub babe_authority: plant_babe::Pallet<Test>,
	}
}

impl plant_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as topsoil_system::Config>::AccountId;
	type ValidatorIdOf = subsoil::runtime::traits::ConvertInto;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = plant_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = MockSessionKeys;
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
	type FindAuthor = plant_session::FindAccountFromAuthorIndex<Self, Babe>;
	type EventHandler = ();
}

impl plant_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = Babe;
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

type Balance = u128;
#[derive_impl(plant_balances::config_preludes::TestDefaultConfig)]
impl plant_balances::Config for Test {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
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
	pub static ElectionsBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<DummyValidatorId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinnersPerPage = ConstU32<100>;
	type MaxBackersPerWinner = ConstU32<100>;
	type Sort = ConstBool<true>;
	type Bounds = ElectionsBounds;
}

#[derive_impl(plant_staking::config_preludes::TestDefaultConfig)]
impl plant_staking::Config for Test {
	type OldCurrency = Balances;
	type Currency = Balances;
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
	type BenchmarkingConfig = plant_staking::TestBenchmarkingConfig;
}

impl plant_offences::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = plant_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

parameter_types! {
	pub const EpochDuration: u64 = 3;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl Config for Test {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ConstU64<1>;
	type EpochChangeTrigger = plant_babe::ExternalTrigger;
	type DisabledValidators = Session;
	type WeightInfo = ();
	type MaxAuthorities = ConstU32<10>;
	type MaxNominators = ConstU32<100>;
	type KeyOwnerProof = subsoil::session::MembershipProof;
	type EquivocationReportSystem =
		plant_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

pub fn go_to_block(n: u64, s: u64) {
	use topsoil_support::traits::OnFinalize;

	Babe::on_finalize(System::block_number());
	Session::on_finalize(System::block_number());
	Staking::on_finalize(System::block_number());

	let parent_hash = if System::block_number() > 1 {
		let hdr = System::finalize();
		hdr.hash()
	} else {
		System::parent_hash()
	};

	let pre_digest = make_secondary_plain_pre_digest(0, s.into());

	System::reset_events();
	System::initialize(&n, &parent_hash, &pre_digest);

	Babe::on_initialize(n);
	Session::on_initialize(n);
	Staking::on_initialize(n);
}

/// Slots will grow accordingly to blocks
pub fn progress_to_block(n: u64) {
	let mut slot = u64::from(CurrentSlot::<Test>::get()) + 1;
	for i in System::block_number() + 1..=n {
		go_to_block(i, slot);
		slot += 1;
	}
}

/// Progress to the first block at the given session
pub fn start_session(session_index: SessionIndex) {
	let missing = (session_index - Session::current_index()) * 3;
	progress_to_block(System::block_number() + missing as u64 + 1);
	assert_eq!(Session::current_index(), session_index);
}

/// Progress to the first block at the given era
pub fn start_era(era_index: EraIndex) {
	start_session((era_index * 3).into());
	assert_eq!(plant_staking::CurrentEra::<Test>::get(), Some(era_index));
}

pub fn make_primary_pre_digest(
	authority_index: subsoil::consensus::babe::AuthorityIndex,
	slot: subsoil::consensus::babe::Slot,
	vrf_signature: VrfSignature,
) -> Digest {
	let digest_data = subsoil::consensus::babe::digests::PreDigest::Primary(
		subsoil::consensus::babe::digests::PrimaryPreDigest {
			authority_index,
			slot,
			vrf_signature,
		},
	);
	let log =
		DigestItem::PreRuntime(subsoil::consensus::babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_secondary_plain_pre_digest(
	authority_index: subsoil::consensus::babe::AuthorityIndex,
	slot: subsoil::consensus::babe::Slot,
) -> Digest {
	let digest_data = subsoil::consensus::babe::digests::PreDigest::SecondaryPlain(
		subsoil::consensus::babe::digests::SecondaryPlainPreDigest { authority_index, slot },
	);
	let log =
		DigestItem::PreRuntime(subsoil::consensus::babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_secondary_vrf_pre_digest(
	authority_index: subsoil::consensus::babe::AuthorityIndex,
	slot: subsoil::consensus::babe::Slot,
	vrf_signature: VrfSignature,
) -> Digest {
	let digest_data = subsoil::consensus::babe::digests::PreDigest::SecondaryVRF(
		subsoil::consensus::babe::digests::SecondaryVRFPreDigest {
			authority_index,
			slot,
			vrf_signature,
		},
	);
	let log =
		DigestItem::PreRuntime(subsoil::consensus::babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_vrf_signature_and_randomness(
	slot: Slot,
	pair: &subsoil::consensus::babe::AuthorityPair,
) -> (VrfSignature, Randomness) {
	let transcript = subsoil::consensus::babe::make_vrf_transcript(
		&plant_babe::Randomness::<Test>::get(),
		slot,
		0,
	);

	let randomness = pair
		.as_ref()
		.make_bytes(subsoil::consensus::babe::RANDOMNESS_VRF_CONTEXT, &transcript);

	let signature = pair.as_ref().vrf_sign(&transcript.into());

	(signature, randomness)
}

pub fn new_test_ext(authorities_len: usize) -> subsoil::io::TestExternalities {
	new_test_ext_with_pairs(authorities_len).1
}

pub fn new_test_ext_with_pairs(
	authorities_len: usize,
) -> (Vec<AuthorityPair>, subsoil::io::TestExternalities) {
	let pairs = (0..authorities_len)
		.map(|i| AuthorityPair::from_seed(&U256::from(i).to_little_endian()))
		.collect::<Vec<_>>();

	let public = pairs.iter().map(|p| p.public()).collect();

	(pairs, new_test_ext_raw_authorities(public))
}

pub fn new_test_ext_raw_authorities(
	authorities: Vec<AuthorityId>,
) -> subsoil::io::TestExternalities {
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
		.map(|(i, k)| {
			(i as u64, i as u64, MockSessionKeys { babe_authority: AuthorityId::from(k.clone()) })
		})
		.collect();

	// NOTE: this will initialize the babe authorities
	// through OneSessionHandler::on_genesis_session
	plant_session::GenesisConfig::<Test> { keys: session_keys, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	// controllers are same as stash
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

/// Creates an equivocation at the current block, by generating two headers.
pub fn generate_equivocation_proof(
	offender_authority_index: u32,
	offender_authority_pair: &AuthorityPair,
	slot: Slot,
) -> subsoil::consensus::babe::EquivocationProof<Header> {
	use subsoil::consensus::babe::digests::CompatibleDigestItem;

	let current_block = System::block_number();
	let current_slot = CurrentSlot::<Test>::get();

	let make_header = || {
		// We don't want to change any state, so we build the headers in a transaction and revert it
		// afterward.
		topsoil_support::storage::with_transaction(|| {
			let parent_hash = System::parent_hash();
			let pre_digest = make_secondary_plain_pre_digest(offender_authority_index, slot);
			System::reset_events();
			System::set_block_number(System::block_number() - 1);
			System::initialize(&current_block, &parent_hash, &pre_digest);
			System::set_block_number(current_block);
			Timestamp::set_timestamp(*current_slot * Babe::slot_duration());
			let header = System::finalize();

			subsoil::runtime::TransactionOutcome::Rollback(Ok::<_, DispatchError>(header))
		})
		.unwrap()
	};

	// Sign the header prehash and sign it, adding it to the block as the seal
	// digest item
	let seal_header = |header: &mut Header| {
		let prehash = header.hash();
		let seal = <DigestItem as CompatibleDigestItem>::babe_seal(
			offender_authority_pair.sign(prehash.as_ref()),
		);
		header.digest_mut().push(seal);
	};

	// Generate two headers at the current block
	let mut h1 = make_header();
	let mut h2 = make_header();

	seal_header(&mut h1);
	seal_header(&mut h2);

	subsoil::consensus::babe::EquivocationProof {
		slot,
		offender: offender_authority_pair.public(),
		first_header: h1,
		second_header: h2,
	}
}

mod tests;
