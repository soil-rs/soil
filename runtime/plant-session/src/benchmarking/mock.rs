// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mock file for session benchmarking.

#![cfg(test)]

use super::MembershipBenchmarkSetup;
use codec::Encode;
use subsoil::runtime::{traits::{IdentityLookup, OpaqueKeys, One}, BuildStorage, KeyTypeId};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, Get, KeyOwnerProofSystem, OnInitialize},
};

use crate::{historical::Pallet as Historical, Pallet as Session, Validators};

type AccountId = u64;
type Nonce = u32;

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: plant_balances,
		Staking: plant_staking,
		Session: plant_session,
		Historical: plant_session::historical
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Nonce = Nonce;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
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

impl plant_session::Config for Test {
	type SessionManager = plant_session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = SessionKeys;
	type ShouldEndSession = plant_session::PeriodicSessions<(), ()>;
	type NextSessionRotation = plant_session::PeriodicSessions<(), ()>;
	type SessionHandler = TestSessionHandler;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = subsoil::runtime::traits::ConvertInto;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ConstU64<2_000_000_000>;
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
	type Solver = SequentialPhragmen<AccountId, subsoil::runtime::Perbill>;
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

impl super::Config for Test {
	fn generate_session_keys_and_proof(owner: Self::AccountId) -> (Self::Keys, Vec<u8>) {
		let keys = SessionKeys::generate(&owner.encode(), None);
		(keys.keys, keys.proof.encode())
	}

	fn setup_benchmark_controller() -> Result<Self::AccountId, &'static str> {
		let max_nominations = plant_staking::MaxNominationsOf::<Self>::get();
		let (stash, _) = plant_staking::benchmarking::create_validator_with_nominators::<Self>(
			max_nominations,
			max_nominations,
			false,
			true,
			plant_staking::RewardDestination::Staked,
		)?;

		plant_staking::Pallet::<Self>::bonded(&stash).ok_or("not stash")
	}

	fn setup_membership_proof_benchmark(n: u32) -> Result<MembershipBenchmarkSetup, &'static str> {
		plant_staking::ValidatorCount::<Self>::put(n);
		let mut first_key = None;

		for who in plant_staking::testing_utils::create_validators::<Self>(n, 1000)? {
			let validator = Self::Lookup::lookup(who).map_err(|_| "lookup failed")?;
			let controller = plant_staking::Pallet::<Self>::bonded(&validator).ok_or("not stash")?;
			let (keys, proof) = Self::generate_session_keys_and_proof(controller.clone());

			if first_key.is_none() {
				let key_type = SessionKeys::key_ids()[0];
				let key_data = keys.get_raw(key_type).to_vec();
				first_key = Some((key_type, key_data));
			}

			Session::<Self>::set_keys(
				topsoil_system::RawOrigin::Signed(controller).into(),
				keys,
				proof,
			)
			.map_err(|_| "failed to set keys")?;
		}

		crate::benchmarking::Pallet::<Self>::on_initialize(One::one());

		while Validators::<Self>::get().len() < n as usize {
			Session::<Self>::rotate_session();
		}

		let key = first_key.ok_or("missing benchmark key")?;
		let proof = Historical::<Self>::prove((key.0, key.1.clone())).ok_or("failed to prove")?;

		Ok((key, proof))
	}
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	subsoil::io::TestExternalities::new(t)
}
