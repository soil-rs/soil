// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Mock file for offences benchmarking.

use codec::Encode;
use subsoil::runtime::{
	testing::{Header, UintAuthorityId},
	BuildStorage, KeyTypeId, Perbill,
};
use plant_election_provider::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use plant_session::historical as pallet_session_historical;
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};
use topsoil_system as system;

type AccountId = u64;

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
	// corresponds to the opaque key id above
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

impl plant_session_benchmarking::Config for Test {
	fn generate_session_keys_and_proof(owner: Self::AccountId) -> (Self::Keys, Vec<u8>) {
		let keys = SessionKeys::generate(&owner.encode(), None);

		(keys.keys, keys.proof.encode())
	}
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
	type IdentificationTuple = plant_session::historical::IdentificationTuple<Self>;
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

impl crate::Config for Test {}

pub type Block = subsoil::runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic =
	subsoil::runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, u64, ()>;

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

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	subsoil::tracing::try_init_simple();
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	subsoil::io::TestExternalities::new(t)
}
