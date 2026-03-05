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

//! Test utilities

#![cfg(test)]

use subsoil::runtime::{testing::UintAuthorityId, traits::ConvertInto, BuildStorage, Permill};
use soil_staking::{
	offence::{OffenceError, ReportOffence},
	SessionIndex,
};
use topsoil_session::historical as pallet_session_historical;
use topsoil_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
	weights::Weight,
};

use crate as imonline;
use crate::Config;

type Block = topsoil_system::mocking::MockBlock<Runtime>;

topsoil_support::construct_runtime!(
	pub enum Runtime {
		System: topsoil_system,
		Session: topsoil_session,
		Balances: topsoil_balances,
		ImOnline: imonline,
		Historical: pallet_session_historical,
	}
);

parameter_types! {
	pub static Validators: Option<Vec<u64>> = Some(vec![
		1,
		2,
		3,
	]);
}

pub struct TestSessionManager;
impl topsoil_session::SessionManager<u64> for TestSessionManager {
	fn new_session(_new_index: SessionIndex) -> Option<Vec<u64>> {
		Validators::mutate(|l| l.take())
	}
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
}

impl topsoil_session::historical::SessionManager<u64, u64> for TestSessionManager {
	fn new_session(_new_index: SessionIndex) -> Option<Vec<(u64, u64)>> {
		Validators::mutate(|l| {
			l.take().map(|validators| validators.iter().map(|v| (*v, *v)).collect())
		})
	}
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
}

/// An extrinsic type used for tests.
pub type Extrinsic = subsoil::runtime::testing::TestXt<RuntimeCall, ()>;
type IdentificationTuple = (u64, u64);
type Offence = crate::UnresponsivenessOffence<IdentificationTuple>;

parameter_types! {
	pub static Offences: Vec<(Vec<u64>, Offence)> = vec![];
}

/// A mock offence report handler.
pub struct OffenceHandler;
impl ReportOffence<u64, IdentificationTuple, Offence> for OffenceHandler {
	fn report_offence(reporters: Vec<u64>, offence: Offence) -> Result<(), OffenceError> {
		Offences::mutate(|l| l.push((reporters, offence)));
		Ok(())
	}

	fn is_known_offence(_offenders: &[IdentificationTuple], _time_slot: &SessionIndex) -> bool {
		false
	}
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
	let mut result: subsoil::io::TestExternalities = t.into();
	// Set the default keys, otherwise session will discard the validator.
	result.execute_with(|| {
		for i in 1..=6 {
			System::inc_providers(&i);
			assert_eq!(Session::set_keys(RuntimeOrigin::signed(i), (i - 1).into(), vec![]), Ok(()));
		}
	});
	result
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Runtime {
	type AccountData = topsoil_balances::AccountData<u64>;
	type Block = Block;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Runtime {
	type AccountStore = System;
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

impl topsoil_session::Config for Runtime {
	type ShouldEndSession = topsoil_session::PeriodicSessions<Period, Offset>;
	type SessionManager =
		topsoil_session::historical::NoteHistoricalRoot<Runtime, TestSessionManager>;
	type SessionHandler = (ImOnline,);
	type ValidatorId = u64;
	type ValidatorIdOf = ConvertInto;
	type Keys = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = topsoil_session::PeriodicSessions<Period, Offset>;
	type DisablingStrategy = ();
	type WeightInfo = ();
	type Currency = Balances;
	type KeyDeposit = ();
}

impl topsoil_session::historical::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = u64;
	type FullIdentificationOf = ConvertInto;
}

impl topsoil_authorship::Config for Runtime {
	type FindAuthor = ();
	type EventHandler = ImOnline;
}

parameter_types! {
	pub static MockCurrentSessionProgress: Option<Option<Permill>> = None;
}

parameter_types! {
	pub static MockAverageSessionLength: Option<u64> = None;
}

pub struct TestNextSessionRotation;

impl topsoil_support::traits::EstimateNextSessionRotation<u64> for TestNextSessionRotation {
	fn average_session_length() -> u64 {
		// take the mock result if any and return it
		let mock = MockAverageSessionLength::mutate(|p| p.take());

		mock.unwrap_or(topsoil_session::PeriodicSessions::<Period, Offset>::average_session_length())
	}

	fn estimate_current_session_progress(now: u64) -> (Option<Permill>, Weight) {
		let (estimate, weight) =
			topsoil_session::PeriodicSessions::<Period, Offset>::estimate_current_session_progress(
				now,
			);

		// take the mock result if any and return it
		let mock = MockCurrentSessionProgress::mutate(|p| p.take());

		(mock.unwrap_or(estimate), weight)
	}

	fn estimate_next_session_rotation(now: u64) -> (Option<u64>, Weight) {
		topsoil_session::PeriodicSessions::<Period, Offset>::estimate_next_session_rotation(now)
	}
}

impl Config for Runtime {
	type AuthorityId = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = TestNextSessionRotation;
	type ReportUnresponsiveness = OffenceHandler;
	type UnsignedPriority = ConstU64<{ 1 << 20 }>;
	type WeightInfo = ();
	type MaxKeys = ConstU32<10_000>;
	type MaxPeerInHeartbeats = ConstU32<10_000>;
}

impl<LocalCall> topsoil_system::offchain::CreateTransactionBase<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	type RuntimeCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> topsoil_system::offchain::CreateBare<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_bare(call: Self::RuntimeCall) -> Self::Extrinsic {
		Extrinsic::new_bare(call)
	}
}

pub fn advance_session() {
	let now = System::block_number().max(1);
	System::set_block_number(now + 1);
	Session::rotate_session();
	let keys = Session::validators().into_iter().map(UintAuthorityId).collect();
	ImOnline::set_keys(keys);
	assert_eq!(Session::current_index(), (now / Period::get()) as u32);
}
