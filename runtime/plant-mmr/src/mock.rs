// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use crate as plant_mmr;
use crate::*;

use crate::{
	primitives::{Compact, LeafDataProvider},
	topsoil_core::system::DefaultConfig,
};
use codec::{Decode, Encode};
use topsoil::{
	deps::topsoil_core::derive_impl,
	prelude::{topsoil_core::system, topsoil_core::system::config_preludes::TestDefaultConfig},
	testing_prelude::*,
};

type Block = MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: topsoil_core::system,
		MMR: plant_mmr,
	}
);

#[derive_impl(TestDefaultConfig)]
impl topsoil_core::system::Config for Test {
	type Block = Block;
}

impl Config for Test {
	const INDEXING_PREFIX: &'static [u8] = b"mmr-";

	type Hashing = Keccak256;
	type LeafData = Compact<Keccak256, (ParentNumberAndHash<Test>, LeafData)>;
	type OnNewRoot = ();
	type BlockHashProvider = DefaultBlockHashProvider<Test>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug)]
pub struct LeafData {
	pub a: u64,
	pub b: Vec<u8>,
}

impl LeafData {
	pub fn new(a: u64) -> Self {
		Self { a, b: Default::default() }
	}
}

parameter_types! {
	pub static LeafDataTestValue: LeafData = Default::default();
}

impl LeafDataProvider for LeafData {
	type LeafData = Self;

	fn leaf_data() -> Self::LeafData {
		LeafDataTestValue::get().clone()
	}
}
