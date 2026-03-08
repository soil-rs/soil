// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The tests for functionality concerning normal starting, ending and enacting of referenda.

use super::*;

#[test]
fn simple_passing_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r, aye(1)));
		assert_eq!(tally(r), Tally { ayes: 1, nays: 0, turnout: 10 });
		assert_eq!(LowestUnbaked::<Test>::get(), 0);
		next_block();
		next_block();
		assert_eq!(LowestUnbaked::<Test>::get(), 1);
		assert_eq!(Balances::free_balance(42), 2);
	});
}

#[test]
fn simple_failing_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r, nay(1)));
		assert_eq!(tally(r), Tally { ayes: 0, nays: 1, turnout: 10 });

		next_block();
		next_block();

		assert_eq!(Balances::free_balance(42), 0);
	});
}

#[test]
fn ooo_inject_referendums_should_work() {
	new_test_ext().execute_with(|| {
		let r1 = Democracy::inject_referendum(
			3,
			set_balance_proposal(3),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		let r2 = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);

		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r2, aye(1)));
		assert_eq!(tally(r2), Tally { ayes: 1, nays: 0, turnout: 10 });

		next_block();

		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r1, aye(1)));
		assert_eq!(tally(r1), Tally { ayes: 1, nays: 0, turnout: 10 });

		next_block();
		assert_eq!(Balances::free_balance(42), 2);

		next_block();
		assert_eq!(Balances::free_balance(42), 3);
	});
}

#[test]
fn delayed_enactment_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			1,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r, aye(1)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(2), r, aye(2)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(3), r, aye(3)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(4), r, aye(4)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(5), r, aye(5)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(6), r, aye(6)));

		assert_eq!(tally(r), Tally { ayes: 21, nays: 0, turnout: 210 });

		next_block();
		assert_eq!(Balances::free_balance(42), 0);

		next_block();
		assert_eq!(Balances::free_balance(42), 2);
	});
}

#[test]
fn lowest_unbaked_should_be_sensible() {
	new_test_ext().execute_with(|| {
		let r1 = Democracy::inject_referendum(
			3,
			set_balance_proposal(1),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		let r2 = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		let r3 = Democracy::inject_referendum(
			10,
			set_balance_proposal(3),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r1, aye(1)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r2, aye(1)));
		// r3 is canceled
		assert_ok!(Democracy::cancel_referendum(RuntimeOrigin::root(), r3.into()));
		assert_eq!(LowestUnbaked::<Test>::get(), 0);

		next_block();
		// r2 ends with approval
		assert_eq!(LowestUnbaked::<Test>::get(), 0);

		next_block();
		// r1 ends with approval
		assert_eq!(LowestUnbaked::<Test>::get(), 3);
		assert_eq!(LowestUnbaked::<Test>::get(), ReferendumCount::<Test>::get());

		// r2 is executed
		assert_eq!(Balances::free_balance(42), 2);

		next_block();
		// r1 is executed
		assert_eq!(Balances::free_balance(42), 1);
	});
}
