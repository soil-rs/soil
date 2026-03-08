// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for remarks pallet.

use super::{Error, Event, Pallet as Remark};
use crate::mock::*;
use topsoil_support::{assert_noop, assert_ok};
use topsoil_system::RawOrigin;

#[test]
fn generates_event() {
	new_test_ext().execute_with(|| {
		let caller = 1;
		let data = vec![0u8; 100];
		System::set_block_number(System::block_number() + 1); // otherwise event won't be registered.
		assert_ok!(Remark::<Test>::store(RawOrigin::Signed(caller).into(), data.clone(),));
		let events = System::events();
		// this one we create as we expect it
		let system_event: <Test as topsoil_system::Config>::RuntimeEvent = Event::Stored {
			content_hash: subsoil::io::hashing::blake2_256(&data).into(),
			sender: caller,
		}
		.into();
		// this one we actually go into the system pallet and get the last event
		// because we know its there from block +1
		let topsoil_system::EventRecord { event, .. } = &events[events.len() - 1];
		assert_eq!(event, &system_event);
	});
}

#[test]
fn does_not_store_empty() {
	new_test_ext().execute_with(|| {
		let caller = 1;
		let data = vec![];
		System::set_block_number(System::block_number() + 1); // otherwise event won't be registered.
		assert_noop!(
			Remark::<Test>::store(RawOrigin::Signed(caller).into(), data.clone(),),
			Error::<Test>::Empty
		);
		assert!(System::events().is_empty());
	});
}
