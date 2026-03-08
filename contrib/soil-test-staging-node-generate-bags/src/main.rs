// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Make the set of bag thresholds to be used with topsoil-bags-list.

use clap::Parser;
use generate_bags::generate_thresholds;
use std::path::PathBuf;

#[derive(Debug, Parser)]
// #[clap(author, version, about)]
struct Opt {
	/// How many bags to generate.
	#[arg(long, default_value_t = 200)]
	n_bags: usize,

	/// Where to write the output.
	output: PathBuf,

	/// The total issuance of the currency used to create `VoteWeight`.
	#[arg(short, long)]
	total_issuance: u128,

	/// The minimum account balance (i.e. existential deposit) for the currency used to create
	/// `VoteWeight`.
	#[arg(short, long)]
	minimum_balance: u128,
}

fn main() -> Result<(), std::io::Error> {
	let Opt { n_bags, output, total_issuance, minimum_balance } = Opt::parse();
	generate_thresholds::<soil_test_staging_node_runtime::Runtime>(
		n_bags,
		&output,
		total_issuance,
		minimum_balance,
	)
}
