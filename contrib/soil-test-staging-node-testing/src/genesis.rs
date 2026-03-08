// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Genesis Configuration.

use crate::keyring::*;
use soil_test_staging_node_runtime::{
	constants::currency::*, AccountId, AssetsConfig, BalancesConfig, IndicesConfig,
	RuntimeGenesisConfig, SessionConfig, SocietyConfig, StakerStatus, StakingConfig,
};
use subsoil::keyring::Ed25519Keyring;
use subsoil::runtime::Perbill;

/// Create genesis runtime configuration for tests.
pub fn config() -> RuntimeGenesisConfig {
	config_endowed(Default::default())
}

/// Create genesis runtime configuration for tests with some extra
/// endowed accounts.
pub fn config_endowed(extra_endowed: Vec<AccountId>) -> RuntimeGenesisConfig {
	let mut endowed = vec![
		(alice(), 111 * DOLLARS),
		(bob(), 100 * DOLLARS),
		(charlie(), 100_000_000 * DOLLARS),
		(dave(), 112 * DOLLARS),
		(eve(), 101 * DOLLARS),
		(ferdie(), 101 * DOLLARS),
	];

	endowed.extend(extra_endowed.into_iter().map(|endowed| (endowed, 100 * DOLLARS)));

	RuntimeGenesisConfig {
		indices: IndicesConfig { indices: vec![] },
		balances: BalancesConfig { balances: endowed, ..Default::default() },
		session: SessionConfig {
			keys: vec![
				(alice(), dave(), session_keys_from_seed(Ed25519Keyring::Alice.into())),
				(bob(), eve(), session_keys_from_seed(Ed25519Keyring::Bob.into())),
				(charlie(), ferdie(), session_keys_from_seed(Ed25519Keyring::Charlie.into())),
			],
			..Default::default()
		},
		staking: StakingConfig {
			stakers: vec![
				(dave(), dave(), 111 * DOLLARS, StakerStatus::Validator),
				(eve(), eve(), 100 * DOLLARS, StakerStatus::Validator),
				(ferdie(), ferdie(), 100 * DOLLARS, StakerStatus::Validator),
			],
			validator_count: 3,
			minimum_validator_count: 0,
			slash_reward_fraction: Perbill::from_percent(10),
			invulnerables: vec![alice(), bob(), charlie()],
			..Default::default()
		},
		society: SocietyConfig { pot: 0 },
		assets: AssetsConfig { assets: vec![(9, alice(), true, 1)], ..Default::default() },
		..Default::default()
	}
}
