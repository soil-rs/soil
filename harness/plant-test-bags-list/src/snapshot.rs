// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test to execute the snapshot using the voter bag.

use remote_externalities::{Builder, Mode, OnlineConfig};
use subsoil::runtime::{
	traits::{Block as BlockT, Zero},
	DeserializeOwned,
};
use plant_election_provider::{
	bounds::{CountBound, DataProviderBounds},
	SortedListProvider,
};
use topsoil_support::traits::PalletInfoAccess;

/// Execute create a snapshot from topsoil-staking.
pub async fn execute<Runtime, Block>(voter_limit: Option<usize>, currency_unit: u64, ws_url: String)
where
	Runtime: crate::RuntimeT<plant_bags_list::Instance1>,
	Block: BlockT + DeserializeOwned,
	Block::Header: DeserializeOwned,
{
	use topsoil_support::storage::generator::StorageMap;

	let mut ext = Builder::<Block>::new()
		.mode(Mode::Online(OnlineConfig {
			transport_uris: vec![ws_url.to_string()],
			// NOTE: we don't scrape topsoil-staking, this kinda ensures that the source of the data
			// is bags-list.
			pallets: vec![
				plant_bags_list::Pallet::<Runtime, plant_bags_list::Instance1>::name()
					.to_string(),
			],
			at: None,
			hashed_prefixes: vec![
				<plant_staking::Bonded<Runtime>>::prefix_hash().to_vec(),
				<plant_staking::Ledger<Runtime>>::prefix_hash().to_vec(),
				<plant_staking::Validators<Runtime>>::map_storage_final_prefix(),
				<plant_staking::Nominators<Runtime>>::map_storage_final_prefix(),
			],
			hashed_keys: vec![
				<plant_staking::Validators<Runtime>>::counter_storage_final_key().to_vec(),
				<plant_staking::Nominators<Runtime>>::counter_storage_final_key().to_vec(),
			],
			..Default::default()
		}))
		.build()
		.await
		.unwrap();

	ext.execute_with(|| {
		use plant_election_provider::ElectionDataProvider;
		log::info!(
			target: crate::LOG_TARGET,
			"{} nodes in bags list.",
			<Runtime as plant_staking::Config>::VoterList::count(),
		);

		let bounds = match voter_limit {
			None => DataProviderBounds::default(),
			Some(v) => DataProviderBounds { count: Some(CountBound(v as u32)), size: None },
		};

		// single page voter snapshot, thus page index == 0.
		let voters =
			<plant_staking::Pallet<Runtime> as ElectionDataProvider>::electing_voters(bounds, Zero::zero())
				.unwrap();

		let mut voters_nominator_only = voters
			.iter()
			.filter(|(v, _, _)| plant_staking::Nominators::<Runtime>::contains_key(v))
			.cloned()
			.collect::<Vec<_>>();
		voters_nominator_only.sort_by_key(|(_, w, _)| *w);

		let currency_unit = currency_unit as f64;
		let min_voter = voters_nominator_only
			.first()
			.map(|(x, y, _)| (x.clone(), *y as f64 / currency_unit));
		let max_voter = voters_nominator_only
			.last()
			.map(|(x, y, _)| (x.clone(), *y as f64 / currency_unit));
		log::info!(
			target: crate::LOG_TARGET,
			"a snapshot with limit {:?} has been created, {} voters are taken. min nominator: {:?}, max: {:?}",
			voter_limit,
			voters.len(),
			min_voter,
			max_voter
		);
	});
}
