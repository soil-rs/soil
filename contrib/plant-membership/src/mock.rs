// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test utilities

use super::*;
use crate as plant_membership;

use subsoil::runtime::{bounded_vec, BuildStorage};

use topsoil_core::{derive_impl, ord_parameter_types, parameter_types, traits::ConstU32};
use topsoil_core::system::EnsureSignedBy;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Membership: plant_membership,
	}
);

parameter_types! {
	pub static Members: Vec<u64> = vec![];
	pub static Prime: Option<u64> = None;
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}
ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
	pub const Three: u64 = 3;
	pub const Four: u64 = 4;
	pub const Five: u64 = 5;
}

pub struct TestChangeMembers;
impl ChangeMembers<u64> for TestChangeMembers {
	fn change_members_sorted(incoming: &[u64], outgoing: &[u64], new: &[u64]) {
		let mut old_plus_incoming = Members::get();
		old_plus_incoming.extend_from_slice(incoming);
		old_plus_incoming.sort();
		let mut new_plus_outgoing = new.to_vec();
		new_plus_outgoing.extend_from_slice(outgoing);
		new_plus_outgoing.sort();
		assert_eq!(old_plus_incoming, new_plus_outgoing);

		Members::set(new.to_vec());
		Prime::set(None);
	}
	fn set_prime(who: Option<u64>) {
		Prime::set(who);
	}
	fn get_prime() -> Option<u64> {
		Prime::get()
	}
}

impl InitializeMembers<u64> for TestChangeMembers {
	fn initialize_members(members: &[u64]) {
		MEMBERS.with(|m| *m.borrow_mut() = members.to_vec());
	}
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureSignedBy<One, u64>;
	type RemoveOrigin = EnsureSignedBy<Two, u64>;
	type SwapOrigin = EnsureSignedBy<Three, u64>;
	type ResetOrigin = EnsureSignedBy<Four, u64>;
	type PrimeOrigin = EnsureSignedBy<Five, u64>;
	type MembershipInitialized = TestChangeMembers;
	type MembershipChanged = TestChangeMembers;
	type MaxMembers = ConstU32<10>;
	type WeightInfo = ();
}

pub(crate) fn new_test_ext() -> subsoil::io::TestExternalities {
	let mut t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	// We use default for brevity, but you can configure as desired if needed.
	plant_membership::GenesisConfig::<Test> {
		members: bounded_vec![10, 20, 30],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

#[cfg(feature = "runtime-benchmarks")]
pub(crate) fn new_bench_ext() -> subsoil::io::TestExternalities {
	topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

#[cfg(feature = "runtime-benchmarks")]
pub(crate) fn clean() {
	Members::set(vec![]);
	Prime::set(None);
}
