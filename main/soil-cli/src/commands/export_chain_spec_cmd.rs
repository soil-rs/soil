// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use crate::error::Result;
use clap::Parser;
use soil_service::{chain_ops, ChainSpec};
use std::{
	fs,
	io::{self, Write},
	path::PathBuf,
};

/// Export a chain-spec to a JSON file in plain or in raw storage format.
///
/// Nodes that expose this command usually have embedded runtimes WASM blobs with
/// genesis config presets which can be referenced via `--chain <id>` . The logic for
/// loading the chain spec into memory based on an `id` is specific to each
/// node and is a prerequisite to enable this command.
///
/// Same functionality can be achieved currently via
/// [`crate::commands::build_spec_cmd::BuildSpecCmd`]  but we recommend
/// `export-chain-spec` in its stead. `build-spec` is known
///  to be a legacy mix of exporting chain specs to JSON files or
///  converting them to raw, which will be better
///  represented under `export-chain-spec`.
#[derive(Debug, Clone, Parser)]
pub struct ExportChainSpecCmd {
	/// The chain spec identifier to export.
	#[arg(long, default_value = "local")]
	pub chain: String,

	/// `chain-spec` JSON file path. If omitted, prints to stdout.
	#[arg(long)]
	pub output: Option<PathBuf>,

	/// Export in raw genesis storage format.
	#[arg(long)]
	pub raw: bool,
}

impl ExportChainSpecCmd {
	/// Run the export-chain-spec command
	pub fn run(&self, spec: Box<dyn ChainSpec>) -> Result<()> {
		let json = chain_ops::build_spec(spec.as_ref(), self.raw)?;
		if let Some(ref path) = self.output {
			fs::write(path, json)?;
			println!("Exported chain spec to {}", path.display());
		} else {
			io::stdout().write_all(json.as_bytes()).map_err(|e| format!("{}", e))?;
		}
		Ok(())
	}
}
