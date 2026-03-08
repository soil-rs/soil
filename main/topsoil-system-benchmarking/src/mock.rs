// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mock file for system benchmarking.

#![cfg(test)]

use codec::Encode;
use subsoil::runtime::BuildStorage;
use topsoil_support::{derive_impl, weights::Weight};

type Block = topsoil_system::mocking::MockBlock<Test>;

topsoil_support::construct_runtime!(
	pub enum Test
	{
		System: topsoil_system,
	}
);

pub struct MockWeights;
impl topsoil_system::ExtensionsWeightInfo for MockWeights {
	fn check_genesis() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_mortality_mortal_transaction() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_mortality_immortal_transaction() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_non_zero_sender() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_nonce() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_spec_version() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_tx_version() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn check_weight() -> Weight {
		Weight::from_parts(10, 0)
	}

	fn weight_reclaim() -> Weight {
		Weight::from_parts(10, 0)
	}
}

#[derive_impl(topsoil_system::config_preludes::TestDefaultConfig)]
impl topsoil_system::Config for Test {
	type Block = Block;
	type ExtensionsWeightInfo = MockWeights;
}

impl crate::Config for Test {}

struct MockedReadRuntimeVersion(Vec<u8>);

impl subsoil::core::traits::ReadRuntimeVersion for MockedReadRuntimeVersion {
	fn read_runtime_version(
		&self,
		_wasm_code: &[u8],
		_ext: &mut dyn subsoil::externalities::Externalities,
	) -> Result<Vec<u8>, String> {
		Ok(self.0.clone())
	}
}

pub fn new_test_ext() -> subsoil::io::TestExternalities {
	let t = topsoil_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	let version = subsoil::version::RuntimeVersion {
		spec_name: "spec_name".into(),
		spec_version: 123,
		impl_version: 456,
		..Default::default()
	};
	let read_runtime_version = MockedReadRuntimeVersion(version.encode());
	let mut ext = subsoil::io::TestExternalities::new(t);
	ext.register_extension(subsoil::core::traits::ReadRuntimeVersionExt::new(read_runtime_version));
	ext
}
