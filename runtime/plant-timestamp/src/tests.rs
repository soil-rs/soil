// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for the Timestamp module.

use crate::mock::*;
use topsoil_core::assert_ok;

#[test]
fn timestamp_works() {
	new_test_ext().execute_with(|| {
		crate::Now::<Test>::put(46);
		assert_ok!(Timestamp::set(RuntimeOrigin::none(), 69));
		assert_eq!(crate::Now::<Test>::get(), 69);
		assert_eq!(Some(69), get_captured_moment());
	});
}

#[docify::export]
#[test]
#[should_panic(expected = "Timestamp must be updated only once in the block")]
fn double_timestamp_should_fail() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(42);
		assert_ok!(Timestamp::set(RuntimeOrigin::none(), 69));
	});
}

#[docify::export]
#[test]
#[should_panic(
	expected = "Timestamp must increment by at least <MinimumPeriod> between sequential blocks"
)]
fn block_period_minimum_enforced() {
	new_test_ext().execute_with(|| {
		crate::Now::<Test>::put(44);
		let _ = Timestamp::set(RuntimeOrigin::none(), 46);
	});
}
