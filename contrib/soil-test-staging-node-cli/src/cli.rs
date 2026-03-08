// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

/// An overarching CLI command definition.
#[derive(Debug, clap::Parser)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: soil_cli::RunCmd,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub mixnet_params: soil_cli::MixnetParams,

	/// Disable automatic hardware benchmarks.
	///
	/// By default these benchmarks are automatically ran at startup and measure
	/// the CPU speed, the memory bandwidth and the disk speed.
	///
	/// The results are then printed out in the logs, and also sent as part of
	/// telemetry, if telemetry is enabled.
	#[arg(long)]
	pub no_hardware_benchmarks: bool,

	/// Number of concurrent workers for statement validation from the network.
	///
	/// Only relevant when `--enable-statement-store` is used.
	#[arg(long, default_value_t = 1)]
	pub statement_network_workers: usize,

	/// Maximum statements per second per peer before rate limiting kicks in.
	///
	/// Uses a token bucket algorithm that allows short bursts up to this limit
	/// while enforcing the average rate over time.
	///
	/// Only relevant when `--enable-statement-store` is used.
	#[arg(long, default_value_t = 50_000)]
	pub statement_rate_limit: u32,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub storage_monitor: soil_client::storage_monitor::StorageMonitorParams,
}

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// The custom inspect subcommand for decoding blocks and extrinsics.
	#[command(
		name = "inspect",
		about = "Decode given block or extrinsic using current native runtime."
	)]
	Inspect(soil_test_staging_node_inspect::cli::InspectCmd),

	/// Key management cli utilities
	#[command(subcommand)]
	Key(soil_cli::KeySubcommand),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(soil_cli::VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(soil_cli::VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(soil_cli::SignCmd),

	/// Build a chain specification.
	/// DEPRECATED: `build-spec` command will be removed after 1/04/2026. Use `export-chain-spec`
	/// command instead.
	#[deprecated(
		note = "build-spec command will be removed after 1/04/2026. Use export-chain-spec command instead"
	)]
	BuildSpec(soil_cli::BuildSpecCmd),

	/// Export the chain specification.
	ExportChainSpec(soil_cli::ExportChainSpecCmd),

	/// Validate blocks.
	CheckBlock(soil_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(soil_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(soil_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(soil_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(soil_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(soil_cli::RevertCmd),

	/// Db meta columns information.
	ChainInfo(soil_cli::ChainInfoCmd),
}
