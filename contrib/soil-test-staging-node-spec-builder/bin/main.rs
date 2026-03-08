// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use chain_spec_builder::ChainSpecBuilder;
use clap::Parser;
use soil_test_staging_node_spec_builder as chain_spec_builder;

// avoid error message escaping
fn main() {
	match inner_main() {
		Err(e) => eprintln!("{}", format!("{e}")),
		_ => {},
	}
}

fn inner_main() -> Result<(), String> {
	subsoil::tracing::try_init_simple();

	let builder = ChainSpecBuilder::parse();
	builder.run()
}
