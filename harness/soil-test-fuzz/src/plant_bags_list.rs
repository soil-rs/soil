// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! # Running
//! Running this fuzzer can be done with `cargo hfuzz run plant_bags_list`. `honggfuzz` CLI
//! options can be used by setting `HFUZZ_RUN_ARGS`, such as `-n 4` to use 4 threads.
//!
//! # Debugging a panic
//! Once a panic is found, it can be debugged with
//! `cargo hfuzz run-debug plant_bags_list hfuzz_workspace/plant_bags_list/*.fuzz`.
//!
//! # More information
//! More information about `honggfuzz` can be found
//! [here](https://docs.rs/honggfuzz/).

use honggfuzz::fuzz;
use plant_bags_list::mock::{AccountId, BagsList, ExtBuilder};
use plant_election_provider::{SortedListProvider, VoteWeight};

const ID_RANGE: AccountId = 25_000;

/// Actions of a `SortedListProvider` that we fuzz.
enum Action {
	Insert,
	Update,
	Remove,
}

impl From<u32> for Action {
	fn from(v: u32) -> Self {
		let num_variants = Self::Remove as u32 + 1;
		match v % num_variants {
			_x if _x == Action::Insert as u32 => Action::Insert,
			_x if _x == Action::Update as u32 => Action::Update,
			_x if _x == Action::Remove as u32 => Action::Remove,
			_ => unreachable!(),
		}
	}
}

fn main() {
	ExtBuilder::default().build_and_execute(|| loop {
		fuzz!(|data: (AccountId, VoteWeight, u32)| {
			let (account_id_seed, vote_weight, action_seed) = data;

			let id = account_id_seed % ID_RANGE;
			let action = Action::from(action_seed);

			match action {
				Action::Insert => {
					if BagsList::on_insert(id, vote_weight).is_err() {
						// this was a duplicate id, which is ok. We can just update it.
						BagsList::on_update(&id, vote_weight).unwrap();
					}
					assert!(BagsList::contains(&id));
				},
				Action::Update => {
					let already_contains = BagsList::contains(&id);
					if already_contains {
						BagsList::on_update(&id, vote_weight).unwrap();
						assert!(BagsList::contains(&id));
					} else {
						BagsList::on_update(&id, vote_weight).unwrap_err();
					}
				},
				Action::Remove => {
					let already_contains = BagsList::contains(&id);
					if already_contains {
						BagsList::on_remove(&id).unwrap();
					} else {
						BagsList::on_remove(&id).unwrap_err();
					}
					assert!(!BagsList::contains(&id));
				},
			}

			assert!(BagsList::do_try_state().is_ok());
		})
	});
}
