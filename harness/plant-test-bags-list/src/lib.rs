// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utilities for remote-testing topsoil-bags-list.

use plant_bags_list::Instance1;
use plant_election_provider::ScoreProvider;

/// A common log target to use.
pub const LOG_TARGET: &str = "runtime::bags-list::remote-tests";

pub mod migration;
pub mod snapshot;
pub mod try_state;

/// A wrapper for a runtime that the functions of this crate expect.
///
/// For example, this can be the `Runtime` type of the Polkadot runtime.
pub trait RuntimeT<I: 'static>:
	plant_staking::Config + plant_bags_list::Config<I> + topsoil_core::system::Config
{
}
impl<
		I: 'static,
		T: plant_staking::Config + plant_bags_list::Config<I> + topsoil_core::system::Config,
	> RuntimeT<I> for T
{
}

fn percent(portion: u32, total: u32) -> f64 {
	(portion as f64 / total as f64) * 100f64
}

/// Display the number of nodes in each bag, while identifying those that need a rebag.
pub fn display_and_check_bags<Runtime: RuntimeT<Instance1>>(
	currency_unit: u64,
	currency_name: &'static str,
) {
	use plant_election_provider::SortedListProvider;
	use topsoil_core::traits::Get;

	let min_nominator_bond = <plant_staking::MinNominatorBond<Runtime>>::get();
	log::info!(target: LOG_TARGET, "min nominator bond is {:?}", min_nominator_bond);

	let voter_list_count = <Runtime as plant_staking::Config>::VoterList::count();

	// go through every bag to track the total number of voters within bags and log some info about
	// how voters are distributed within the bags.
	let mut seen_in_bags = 0;
	let mut rebaggable = 0;
	let mut active_bags = 0;
	for vote_weight_thresh in
		<Runtime as plant_bags_list::Config<Instance1>>::BagThresholds::get()
	{
		let vote_weight_thresh_u64: u64 = (*vote_weight_thresh)
			.try_into()
			.map_err(|_| "runtime must configure score to at most u64 to use this test")
			.unwrap();
		// threshold in terms of UNITS (e.g. KSM, DOT etc)
		let vote_weight_thresh_as_unit = vote_weight_thresh_u64 as f64 / currency_unit as f64;
		let pretty_thresh = format!("Threshold: {}. {}", vote_weight_thresh_as_unit, currency_name);

		let bag = match plant_bags_list::Pallet::<Runtime, Instance1>::list_bags_get(
			*vote_weight_thresh,
		) {
			Some(bag) => bag,
			None => {
				log::info!(target: LOG_TARGET, "{} NO VOTERS.", pretty_thresh);
				continue;
			},
		};

		active_bags += 1;

		for id in bag.std_iter().map(|node| node.std_id().clone()) {
			let vote_weight =
				<Runtime as plant_bags_list::Config<Instance1>>::ScoreProvider::score(&id)
					.unwrap();
			let vote_weight_thresh_u64: u64 = (*vote_weight_thresh)
				.try_into()
				.map_err(|_| "runtime must configure score to at most u64 to use this test")
				.unwrap();
			let vote_weight_as_balance: plant_staking::BalanceOf<Runtime> =
				vote_weight_thresh_u64.try_into().map_err(|_| "can't convert").unwrap();

			if vote_weight_as_balance < min_nominator_bond {
				log::trace!(
					target: LOG_TARGET,
					"⚠️ {} Account found below min bond: {:?}.",
					pretty_thresh,
					id
				);
			}

			let node = plant_bags_list::Node::<Runtime, Instance1>::get(&id)
				.expect("node in bag must exist.");
			if node.is_misplaced(vote_weight) {
				rebaggable += 1;
				let notional_bag = plant_bags_list::notional_bag_for::<Runtime, _>(vote_weight);
				let notional_bag_as_u64: u64 = notional_bag
					.try_into()
					.map_err(|_| "runtime must configure score to at most u64 to use this test")
					.unwrap();
				log::trace!(
					target: LOG_TARGET,
					"Account {:?} can be rebagged from {:?} to {:?}",
					id,
					vote_weight_thresh_as_unit,
					notional_bag_as_u64 as f64 / currency_unit as f64
				);
			}
		}

		// update our overall counter
		let voters_in_bag = bag.std_iter().count() as u32;
		seen_in_bags += voters_in_bag;

		// percentage of all nominators
		let percent_of_voters = percent(voters_in_bag, voter_list_count);

		log::info!(
			target: LOG_TARGET,
			"{} Nominators: {} [%{:.3}]",
			pretty_thresh,
			voters_in_bag,
			percent_of_voters,
		);
	}

	if seen_in_bags != voter_list_count {
		log::error!(
			target: LOG_TARGET,
			"bags list population ({}) not on par whoever is voter_list ({})",
			seen_in_bags,
			voter_list_count,
		)
	}

	log::info!(
		target: LOG_TARGET,
		"a total of {} nodes are in {} active bags [{} total bags], {} of which can be rebagged.",
		voter_list_count,
		active_bags,
		<Runtime as plant_bags_list::Config<Instance1>>::BagThresholds::get().len(),
		rebaggable,
	);
}
