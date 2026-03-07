// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Tests for topsoil-dev-mode.

use crate::*;
use subsoil::core::H256;
use subsoil::runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use topsoil_support::{assert_ok, derive_impl};
// Reexport crate as its pallet name for construct_runtime.
use crate as plant_dev_mode;

type Block = topsoil_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct a mock runtime.
topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
		Balances: topsoil_balances,
		Example: plant_dev_mode,
	}
);

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type BaseCallFilter = topsoil_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type Hash = H256;
	type RuntimeCall = RuntimeCall;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = topsoil_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = topsoil_support::traits::ConstU32<16>;
}

#[derive_impl(topsoil_balances::config_preludes::TestDefaultConfig)]
impl topsoil_balances::Config for Test {
	type AccountStore = System;
}

impl Config for Test {}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = RuntimeGenesisConfig {
		// We use default for brevity, but you can configure as desired if needed.
		system: Default::default(),
		balances: Default::default(),
	}
	.build_storage()
	.unwrap();
	t.into()
}

#[test]
fn it_works_for_optional_value() {
	new_test_ext().execute_with(|| {
		assert_eq!(Dummy::<Test>::get(), None);

		let val1 = 42;
		assert_ok!(Example::add_dummy(RuntimeOrigin::root(), val1));
		assert_eq!(Dummy::<Test>::get(), Some(vec![val1]));

		// Check that accumulate works when we have Some value in Dummy already.
		let val2 = 27;
		assert_ok!(Example::add_dummy(RuntimeOrigin::root(), val2));
		assert_eq!(Dummy::<Test>::get(), Some(vec![val1, val2]));
	});
}

#[test]
fn set_dummy_works() {
	new_test_ext().execute_with(|| {
		let test_val = 133;
		assert_ok!(Example::set_bar(RuntimeOrigin::signed(1), test_val.into()));
		assert_eq!(Bar::<Test>::get(1), Some(test_val));
	});
}
