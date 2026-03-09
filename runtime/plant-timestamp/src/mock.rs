// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests Utilities.

use super::*;
use crate as plant_timestamp;

use subsoil::io::TestExternalities;
use subsoil::runtime::BuildStorage;
use topsoil_core::{derive_impl, parameter_types, traits::ConstU64};

type Block = topsoil_core::system::mocking::MockBlock<Test>;
type Moment = u64;

topsoil_core::construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		Timestamp: plant_timestamp,
	}
);

#[derive_impl(topsoil_core::system::config_preludes::TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}

parameter_types! {
	pub static CapturedMoment: Option<Moment> = None;
}

pub struct MockOnTimestampSet;
impl OnTimestampSet<Moment> for MockOnTimestampSet {
	fn on_timestamp_set(moment: Moment) {
		CapturedMoment::mutate(|x| *x = Some(moment));
	}
}

impl Config for Test {
	type Moment = Moment;
	type OnTimestampSet = MockOnTimestampSet;
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

pub(crate) fn clear_captured_moment() {
	CapturedMoment::mutate(|x| *x = None);
}

pub(crate) fn get_captured_moment() -> Option<Moment> {
	CapturedMoment::get()
}

pub(crate) fn new_test_ext() -> TestExternalities {
	let t = topsoil_core::system::GenesisConfig::<Test>::default().build_storage().unwrap();
	clear_captured_moment();
	TestExternalities::new(t)
}
