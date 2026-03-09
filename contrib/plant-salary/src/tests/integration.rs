// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The crate's tests.

use crate as plant_salary;
use crate::*;
use topsoil::{deps::io, testing_prelude::*};
use plant_ranked_collective::{EnsureRanked, Geometric};

type Rank = u16;
type Block = topsoil_core::system::mocking::MockBlock<Test>;

construct_runtime!(
	pub struct Test {
		System: topsoil_core::system,
		Salary: plant_salary,
		Club: plant_ranked_collective,
	}
);

parameter_types! {
	pub BlockWeights: topsoil_core::system::limits::BlockWeights =
		topsoil_core::system::limits::BlockWeights::simple_max(Weight::from_parts(1_000_000, 0));
}

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}

pub struct MinRankOfClass<Delta>(PhantomData<Delta>);
impl<Delta: Get<Rank>> Convert<u16, Rank> for MinRankOfClass<Delta> {
	fn convert(a: u16) -> Rank {
		a.saturating_sub(Delta::get())
	}
}

pub struct TestPay;
impl Pay for TestPay {
	type Beneficiary = u64;
	type Balance = u64;
	type Id = u64;
	type AssetKind = ();
	type Error = ();

	fn pay(
		_: &Self::Beneficiary,
		_: Self::AssetKind,
		_: Self::Balance,
	) -> Result<Self::Id, Self::Error> {
		unreachable!()
	}
	fn check_payment(_: Self::Id) -> PaymentStatus {
		unreachable!()
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_successful(_: &Self::Beneficiary, _: Self::AssetKind, _: Self::Balance) {}
	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_concluded(_: Self::Id) {
		unreachable!()
	}
}

parameter_types! {
	pub static Budget: u64 = 10;
}

impl Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Paymaster = TestPay;
	type Members = Club;
	type Salary = FixedSalary;
	type RegistrationPeriod = ConstU64<2>;
	type PayoutPeriod = ConstU64<2>;
	type Budget = Budget;
}

pub struct FixedSalary;
impl GetSalary<u16, u64, u64> for FixedSalary {
	fn get_salary(_rank: u16, _who: &u64) -> u64 {
		123
	}
}

parameter_types! {
	pub static MinRankOfClassDelta: Rank = 0;
}

impl plant_ranked_collective::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type PromoteOrigin = EitherOf<
		// Root can promote arbitrarily.
		topsoil_core::system::EnsureRootWithSuccess<Self::AccountId, ConstU16<65535>>,
		// Members can promote up to the rank of 2 below them.
		MapSuccess<EnsureRanked<Test, (), 2>, ReduceBy<ConstU16<2>>>,
	>;
	type AddOrigin = MapSuccess<Self::PromoteOrigin, ReplaceWithDefault<()>>;
	type DemoteOrigin = EitherOf<
		// Root can demote arbitrarily.
		topsoil_core::system::EnsureRootWithSuccess<Self::AccountId, ConstU16<65535>>,
		// Members can demote up to the rank of 3 below them.
		MapSuccess<EnsureRanked<Test, (), 3>, ReduceBy<ConstU16<3>>>,
	>;
	type RemoveOrigin = Self::DemoteOrigin;
	type ExchangeOrigin = EitherOf<
		// Root can exchange arbitrarily.
		topsoil_core::system::EnsureRootWithSuccess<Self::AccountId, ConstU16<65535>>,
		// Members can exchange up to the rank of 2 below them.
		MapSuccess<EnsureRanked<Test, (), 2>, ReduceBy<ConstU16<2>>>,
	>;
	type Polls = NoOpPoll<BlockNumberFor<Test>>;
	type MinRankOfClass = MinRankOfClass<MinRankOfClassDelta>;
	type MemberSwappedHandler = Salary;
	type VoteWeight = Geometric;
	type MaxMemberCount = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkSetup = Salary;
}

pub fn new_test_ext() -> TestState {
	let t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let mut ext = TestState::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn assert_last_event(generic_event: <Test as Config>::RuntimeEvent) {
	let events = topsoil_core::system::Pallet::<Test>::events();
	let system_event: <Test as topsoil_core::system::Config>::RuntimeEvent = generic_event.into();
	let topsoil_core::system::EventRecord { event, .. } = events.last().expect("Event expected");
	assert_eq!(event, &system_event.into());
}

fn promote_n_times(acc: u64, r: u16) {
	for _ in 0..r {
		assert_ok!(Club::promote_member(RuntimeOrigin::root(), acc));
	}
}

#[test]
fn swap_simple_works() {
	new_test_ext().execute_with(|| {
		for i in 0u16..9 {
			let acc = i as u64;

			assert_ok!(Club::add_member(RuntimeOrigin::root(), acc));
			promote_n_times(acc, i);
			let _ = Salary::init(RuntimeOrigin::signed(acc));
			assert_ok!(Salary::induct(RuntimeOrigin::signed(acc)));

			// Swapping normally works:
			assert_ok!(Club::exchange_member(RuntimeOrigin::root(), acc, acc + 10));
			assert_last_event(Event::Swapped { who: acc, new_who: acc + 10 }.into());
		}
	});
}

#[test]
fn swap_exhaustive_works() {
	new_test_ext().execute_with(|| {
		let root_add = hypothetically!({
			assert_ok!(Club::add_member(RuntimeOrigin::root(), 1));
			assert_ok!(Club::promote_member(RuntimeOrigin::root(), 1));
			assert_ok!(Salary::init(RuntimeOrigin::signed(1)));
			assert_ok!(Salary::induct(RuntimeOrigin::signed(1)));

			// The events mess up the storage root:
			System::reset_events();
			subsoil::io::storage::root(StateVersion::V1)
		});

		let root_swap = hypothetically!({
			assert_ok!(Club::add_member(RuntimeOrigin::root(), 0));
			assert_ok!(Club::promote_member(RuntimeOrigin::root(), 0));
			assert_ok!(Salary::init(RuntimeOrigin::signed(0)));
			assert_ok!(Salary::induct(RuntimeOrigin::signed(0)));

			assert_ok!(Club::exchange_member(RuntimeOrigin::root(), 0, 1));

			// The events mess up the storage root:
			System::reset_events();
			subsoil::io::storage::root(StateVersion::V1)
		});

		assert_eq!(root_add, root_swap);
		// Ensure that we don't compare trivial stuff like `()` from a type error above.
		assert_eq!(root_add.len(), 32);
	});
}

#[test]
fn swap_bad_noops() {
	new_test_ext().execute_with(|| {
		assert_ok!(Club::add_member(RuntimeOrigin::root(), 0));
		promote_n_times(0, 0);
		assert_ok!(Salary::init(RuntimeOrigin::signed(0)));
		assert_ok!(Salary::induct(RuntimeOrigin::signed(0)));
		assert_ok!(Club::add_member(RuntimeOrigin::root(), 1));
		promote_n_times(1, 1);
		assert_ok!(Salary::induct(RuntimeOrigin::signed(1)));

		// Swapping for another member is a noop:
		assert_noop!(
			Club::exchange_member(RuntimeOrigin::root(), 0, 1),
			plant_ranked_collective::Error::<Test>::AlreadyMember
		);
		// Swapping for the same member is a noop:
		assert_noop!(
			Club::exchange_member(RuntimeOrigin::root(), 0, 0),
			plant_ranked_collective::Error::<Test>::SameMember
		);
	});
}
