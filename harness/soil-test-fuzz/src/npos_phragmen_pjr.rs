// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Fuzzing which ensures that running unbalanced sequential phragmen always produces a result
//! which satisfies our PJR checker.
//!
//! ## Running a single iteration
//!
//! Honggfuzz shuts down each individual loop iteration after a configurable time limit.
//! It can be helpful to run a single iteration on your hardware to help benchmark how long that
//! time limit should reasonably be. Simply run the program without the `fuzzing` configuration to
//! run a single iteration: `cargo run -p soil-test-fuzz --bin npos_phragmen_pjr`.
//!
//! ## Running
//!
//! Run with `HFUZZ_RUN_ARGS="-t 10" cargo hfuzz run npos_phragmen_pjr`.
//!
//! Note the environment variable: by default, `cargo hfuzz` shuts down each iteration after 1
//! second of runtime. We significantly increase that to ensure that the fuzzing gets a chance to
//! complete. Running a single iteration can help determine an appropriate value for this parameter.
//!
//! ## Debugging a panic
//!
//! Once a panic is found, it can be debugged with
//! `HFUZZ_RUN_ARGS="-t 10" cargo hfuzz run-debug npos_phragmen_pjr hfuzz_workspace/npos_phragmen_pjr/*.fuzz`.

#[cfg(fuzzing)]
use honggfuzz::fuzz;

#[cfg(not(fuzzing))]
use clap::Parser;

mod npos_common;
use npos_common::{generate_random_npos_inputs, to_range};
use rand::{self, SeedableRng};
use subsoil::npos_elections::{
	pjr_check_core, seq_phragmen_core, setup_inputs, standard_threshold,
};

type AccountId = u64;

const MIN_CANDIDATES: usize = 250;
const MAX_CANDIDATES: usize = 1000;
const MIN_VOTERS: usize = 500;
const MAX_VOTERS: usize = 2500;

#[cfg(fuzzing)]
fn main() {
	loop {
		fuzz!(|data: (usize, usize, u64)| {
			let (candidate_count, voter_count, seed) = data;
			iteration(candidate_count, voter_count, seed);
		});
	}
}

#[cfg(not(fuzzing))]
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Opt {
	/// How many candidates participate in this election
	#[arg(short, long)]
	candidates: Option<usize>,

	/// How many voters participate in this election
	#[arg(short, long)]
	voters: Option<usize>,

	/// Random seed to use in this election
	#[arg(long)]
	seed: Option<u64>,
}

#[cfg(not(fuzzing))]
fn main() {
	let opt = Opt::parse();
	// candidates and voters by default use the maxima, which turn out to be one less than
	// the constant.
	iteration(
		opt.candidates.unwrap_or(MAX_CANDIDATES - 1),
		opt.voters.unwrap_or(MAX_VOTERS - 1),
		opt.seed.unwrap_or_default(),
	);
}

fn iteration(mut candidate_count: usize, mut voter_count: usize, seed: u64) {
	let rng = rand::rngs::SmallRng::seed_from_u64(seed);
	candidate_count = to_range(candidate_count, MIN_CANDIDATES, MAX_CANDIDATES);
	voter_count = to_range(voter_count, MIN_VOTERS, MAX_VOTERS);

	let (rounds, candidates, voters) =
		generate_random_npos_inputs(candidate_count, voter_count, rng);

	let (candidates, voters) = setup_inputs(candidates, voters);

	// Run seq-phragmen
	let (candidates, voters) = seq_phragmen_core::<AccountId>(rounds, candidates, voters)
		.expect("seq_phragmen must succeed");

	let threshold = standard_threshold(rounds, voters.iter().map(|voter| voter.budget()));

	assert!(
		pjr_check_core(&candidates, &voters, threshold).is_ok(),
		"unbalanced sequential phragmen must satisfy PJR",
	);
}
