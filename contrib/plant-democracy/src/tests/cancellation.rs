// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The tests for cancellation functionality.

use super::*;

#[test]
fn cancel_referendum_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(1), r, aye(1)));
		assert_ok!(Democracy::cancel_referendum(RuntimeOrigin::root(), r.into()));
		assert_eq!(LowestUnbaked::<Test>::get(), 0);

		next_block();

		next_block();

		assert_eq!(LowestUnbaked::<Test>::get(), 1);
		assert_eq!(LowestUnbaked::<Test>::get(), ReferendumCount::<Test>::get());
		assert_eq!(Balances::free_balance(42), 0);
	});
}

#[test]
fn emergency_cancel_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			2,
		);
		assert!(Democracy::referendum_status(r).is_ok());

		assert_noop!(Democracy::emergency_cancel(RuntimeOrigin::signed(3), r), BadOrigin);
		assert_ok!(Democracy::emergency_cancel(RuntimeOrigin::signed(4), r));
		assert!(ReferendumInfoOf::<Test>::get(r).is_none());

		// some time later...

		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			2,
		);
		assert!(Democracy::referendum_status(r).is_ok());
		assert_noop!(
			Democracy::emergency_cancel(RuntimeOrigin::signed(4), r),
			Error::<Test>::AlreadyCanceled,
		);
	});
}
