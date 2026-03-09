// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mock helpers for Session.

use super::*;
use crate as plant_session;
#[cfg(feature = "historical")]
use crate::historical as pallet_session_historical;

use codec::Encode;
use subsoil::staking::SessionIndex;
use std::collections::BTreeMap;
use subsoil::core::crypto::key_types::DUMMY;
use subsoil::runtime::{
	testing::UintAuthorityId,
	traits::{Convert, OpaqueKeys},
	BuildStorage,
};
use plant_balances::{self, AccountData};
use topsoil_core::{derive_impl, parameter_types, traits::ConstU64};

subsoil::impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub dummy: UintAuthorityId,
	}
}

impl From<UintAuthorityId> for MockSessionKeys {
	fn from(dummy: UintAuthorityId) -> Self {
		Self { dummy }
	}
}

pub const KEY_ID_A: KeyTypeId = KeyTypeId([4; 4]);
pub const KEY_ID_B: KeyTypeId = KeyTypeId([9; 4]);

#[derive(Debug, Clone, codec::Encode, codec::Decode, PartialEq, Eq)]
pub struct PreUpgradeMockSessionKeys {
	pub a: [u8; 32],
	pub b: [u8; 64],
}

impl OpaqueKeys for PreUpgradeMockSessionKeys {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [KeyTypeId] {
		&[KEY_ID_A, KEY_ID_B]
	}

	fn get_raw(&self, i: KeyTypeId) -> &[u8] {
		match i {
			i if i == KEY_ID_A => &self.a[..],
			i if i == KEY_ID_B => &self.b[..],
			_ => &[],
		}
	}

	fn ownership_proof_is_valid(&self, _: &[u8], _: &[u8]) -> bool {
		true
	}
}

type Block = topsoil_core::system::mocking::MockBlock<Test>;

#[cfg(feature = "historical")]
topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Session: plant_session,
		Balances: plant_balances,
		Historical: pallet_session_historical,
	}
);

#[cfg(not(feature = "historical"))]
topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Session: plant_session,
		Balances: plant_balances,
	}
);

parameter_types! {
	pub static Validators: Vec<u64> = vec![1, 2, 3];
	pub static NextValidators: Vec<u64> = vec![1, 2, 3];
	pub static Authorities: Vec<UintAuthorityId> =
		vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)];
	pub static ForceSessionEnd: bool = false;
	pub static SessionLength: u64 = 2;
	pub static SessionChanged: bool = false;
	pub static TestSessionChanged: bool = false;
	pub static Disabled: bool = false;
	// Stores if `on_before_session_end` was called
	pub static BeforeSessionEndCalled: bool = false;
	pub static ValidatorAccounts: BTreeMap<u64, u64> = BTreeMap::new();
	pub static KeyDeposit: u64 = 10;
}

pub struct TestShouldEndSession;
impl ShouldEndSession<u64> for TestShouldEndSession {
	fn should_end_session(now: u64) -> bool {
		let l = SessionLength::get();
		now.is_multiple_of(l)
			|| ForceSessionEnd::mutate(|l| {
				let r = *l;
				*l = false;
				r
			})
	}
}

pub struct TestSessionHandler;
impl SessionHandler<u64> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [subsoil::runtime::KeyTypeId] = &[UintAuthorityId::ID];
	fn on_genesis_session<T: OpaqueKeys>(_validators: &[(u64, T)]) {}
	fn on_new_session<T: OpaqueKeys>(
		changed: bool,
		validators: &[(u64, T)],
		_queued_validators: &[(u64, T)],
	) {
		SessionChanged::mutate(|l| *l = changed);
		Authorities::mutate(|l| {
			*l = validators
				.iter()
				.map(|(_, id)| id.get::<UintAuthorityId>(DUMMY).unwrap_or_default())
				.collect()
		});
	}
	fn on_disabled(_validator_index: u32) {
		Disabled::mutate(|l| *l = true)
	}
	fn on_before_session_ending() {
		BeforeSessionEndCalled::mutate(|b| *b = true);
	}
}

pub struct TestSessionManager;
impl SessionManager<u64> for TestSessionManager {
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
	fn new_session(_: SessionIndex) -> Option<Vec<u64>> {
		if !TestSessionChanged::get() {
			Validators::mutate(|v| {
				*v = NextValidators::get().clone();
				Some(v.clone())
			})
		} else if Disabled::mutate(|l| std::mem::replace(&mut *l, false)) {
			// If there was a disabled validator, underlying conditions have changed
			// so we return `Some`.
			Some(Validators::get().clone())
		} else {
			None
		}
	}
}

#[cfg(feature = "historical")]
impl crate::historical::SessionManager<u64, u64> for TestSessionManager {
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
	fn new_session(new_index: SessionIndex) -> Option<Vec<(u64, u64)>> {
		<Self as SessionManager<_>>::new_session(new_index)
			.map(|vals| vals.into_iter().map(|val| (val, val)).collect())
	}
}

pub fn authorities() -> Vec<UintAuthorityId> {
	Authorities::get().to_vec()
}

pub fn force_new_session() {
	ForceSessionEnd::mutate(|l| *l = true)
}

pub fn set_session_length(x: u64) {
	SessionLength::mutate(|l| *l = x)
}

pub fn session_changed() -> bool {
	SessionChanged::get()
}

pub fn set_next_validators(next: Vec<u64>) {
	NextValidators::mutate(|v| *v = next);
}

pub fn before_session_end_called() -> bool {
	BeforeSessionEndCalled::get()
}

pub fn reset_before_session_end_called() {
	BeforeSessionEndCalled::mutate(|b| *b = false);
}

pub fn create_set_keys_proof(owner: u64, public: &UintAuthorityId) -> Vec<u8> {
	public.sign(&owner.encode()).unwrap().encode()
}

parameter_types! {
	pub static LastSessionEventIndex: usize = 0;
}

pub fn session_events_since_last_call() -> Vec<plant_session::Event<Test>> {
	let events = System::read_events_for_pallet::<plant_session::Event<Test>>();
	let already_seen = LastSessionEventIndex::get();
	LastSessionEventIndex::set(events.len());
	events.into_iter().skip(already_seen).collect()
}

pub fn session_hold(who: u64) -> u64 {
	<Balances as topsoil_core::traits::fungible::InspectHold<_>>::balance_on_hold(
		&crate::HoldReason::Keys.into(),
		&who,
	)
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let ed = <Test as plant_balances::Config>::ExistentialDeposit::get();
	plant_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, (KeyDeposit::get() * 10).max(ed)),
			(2, (KeyDeposit::get() * 10).max(ed)),
			(3, (KeyDeposit::get() * 10).max(ed)),
			(4, (KeyDeposit::get() * 10).max(ed)),
			(69, (KeyDeposit::get() * 10).max(ed)),
			// one account who does not have enough balance to pay the key deposit
			(999, (KeyDeposit::get().saturating_sub(1)).max(ed)),
			(1000, (KeyDeposit::get() * 10).max(ed)),
		],
		dev_accounts: None,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let keys: Vec<_> = NextValidators::get()
		.iter()
		.cloned()
		.map(|i| (i, i, UintAuthorityId(i).into()))
		.collect();
	plant_session::GenesisConfig::<Test> { keys, ..Default::default() }
		.assimilate_storage(&mut t)
		.unwrap();

	let v = NextValidators::get().iter().map(|&i| (i, i)).collect();
	ValidatorAccounts::mutate(|m| *m = v);
	subsoil::io::TestExternalities::new(t)
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
	type AccountData = AccountData<u64>;
	type RuntimeEvent = RuntimeEvent;
}

impl plant_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

pub struct TestValidatorIdOf;
impl TestValidatorIdOf {
	pub fn set(v: BTreeMap<u64, u64>) {
		ValidatorAccounts::mutate(|m| *m = v);
	}
}
impl Convert<u64, Option<u64>> for TestValidatorIdOf {
	fn convert(x: u64) -> Option<u64> {
		ValidatorAccounts::get().get(&x).cloned()
	}
}

// Disabling threshold for `UpToLimitDisablingStrategy` and
// `UpToLimitWithReEnablingDisablingStrategy``
pub(crate) const DISABLING_LIMIT_FACTOR: usize = 3;

impl Config for Test {
	type ShouldEndSession = TestShouldEndSession;
	#[cfg(feature = "historical")]
	type SessionManager = crate::historical::NoteHistoricalRoot<Test, TestSessionManager>;
	#[cfg(not(feature = "historical"))]
	type SessionManager = TestSessionManager;
	type SessionHandler = TestSessionHandler;
	type ValidatorId = u64;
	type ValidatorIdOf = TestValidatorIdOf;
	type Keys = MockSessionKeys;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = ();
	type DisablingStrategy =
		disabling::UpToLimitWithReEnablingDisablingStrategy<DISABLING_LIMIT_FACTOR>;
	type WeightInfo = ();
	type Currency = plant_balances::Pallet<Test>;
	type KeyDeposit = KeyDeposit;
}

#[cfg(feature = "historical")]
impl crate::historical::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = u64;
	type FullIdentificationOf = subsoil::runtime::traits::ConvertInto;
}

#[derive_impl(plant_balances::config_preludes::TestDefaultConfig as plant_balances::DefaultConfig)]
impl plant_balances::Config for Test {
	type AccountStore = System;
	type RuntimeEvent = RuntimeEvent;
}
