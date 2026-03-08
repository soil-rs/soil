// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test to execute the sanity-check of the voter bag.

use remote_externalities::{Builder, Mode, OnlineConfig};
use subsoil::runtime::{traits::Block as BlockT, DeserializeOwned};
use topsoil_support::{
	storage::generator::StorageMap,
	traits::{Get, PalletInfoAccess},
};

/// Execute the sanity check of the bags-list.
pub async fn execute<Runtime, Block>(
	currency_unit: u64,
	currency_name: &'static str,
	ws_url: String,
) where
	Runtime: crate::RuntimeT<plant_bags_list::Instance1>,
	Block: BlockT + DeserializeOwned,
	Block::Header: DeserializeOwned,
{
	let mut ext = Builder::<Block>::new()
		.mode(Mode::Online(OnlineConfig {
			transport_uris: vec![ws_url.to_string()],
			pallets: vec![
				plant_bags_list::Pallet::<Runtime, plant_bags_list::Instance1>::name()
					.to_string(),
			],
			hashed_prefixes: vec![
				<plant_staking::Bonded<Runtime>>::prefix_hash().to_vec(),
				<plant_staking::Ledger<Runtime>>::prefix_hash().to_vec(),
			],
			..Default::default()
		}))
		.build()
		.await
		.unwrap();

	ext.execute_with(|| {
		subsoil::core::crypto::set_default_ss58_version(
			Runtime::SS58Prefix::get().try_into().unwrap(),
		);

		plant_bags_list::Pallet::<Runtime, plant_bags_list::Instance1>::do_try_state().unwrap();

		log::info!(target: crate::LOG_TARGET, "executed bags-list sanity check with no errors.");

		crate::display_and_check_bags::<Runtime>(currency_unit, currency_name);
	});
}
