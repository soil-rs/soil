// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Helpers for tests.

#![cfg(feature = "std")]

use crate::{paged_list::StoragePagedListMeta, Config, ListPrefix};
use topsoil::testing_prelude::*;

type Block = topsoil_core::system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test {
		System: topsoil_core::system,
		PagedList: crate,
		PagedList2: crate::<Instance2>,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Nonce = u64;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub storage ValuesPerNewPage: u32 = 5;
	pub const MaxPages: Option<u32> = Some(20);
}

impl crate::Config for Test {
	type Value = u32;
	type ValuesPerNewPage = ValuesPerNewPage;
}

impl crate::Config<crate::Instance2> for Test {
	type Value = u32;
	type ValuesPerNewPage = ValuesPerNewPage;
}

pub type MetaOf<T, I> =
	StoragePagedListMeta<ListPrefix<T, I>, <T as Config>::Value, <T as Config>::ValuesPerNewPage>;

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestState {
	topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

/// Run this closure in test externalities.
pub fn test_closure<R>(f: impl FnOnce() -> R) -> R {
	let mut ext = new_test_ext();
	ext.execute_with(f)
}
