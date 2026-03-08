// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(unix)]
use assert_cmd::cargo::cargo_bin;
use nix::sys::signal::Signal::{self, SIGINT, SIGTERM};
use std::{
	process::{self, Command},
	time::Duration,
};
use tempfile::tempdir;

use soil_test_staging_node_cli_test_utils as common;

#[tokio::test]
async fn running_the_node_works_and_can_be_interrupted() {
	common::run_with_timeout(Duration::from_secs(60 * 10), async move {
		async fn run_command_and_kill(signal: Signal) {
			let base_path = tempdir().expect("could not create a temp dir");
			let rpc_port = common::find_free_tcp_port();
			let mut cmd = common::KillChildOnDrop(
				Command::new(cargo_bin("soil-test-staging-node"))
					.stdout(process::Stdio::null())
					.stderr(process::Stdio::null())
					.args(&["--dev", "-d"])
					.arg(base_path.path())
					.arg("--rpc-port")
					.arg(rpc_port.to_string())
					.arg("--db=paritydb")
					.arg("--no-hardware-benchmarks")
					.spawn()
					.unwrap(),
			);

			common::wait_n_finalized_blocks(3, &common::ws_url_from_port(rpc_port)).await;

			cmd.assert_still_running();

			cmd.stop_with_signal(signal);

			// Check if the database was closed gracefully. If it was not,
			// there may exist a ref cycle that prevents the Client from being dropped properly.
			//
			// parity-db only writes the stats file on clean shutdown.
			let stats_file = base_path.path().join("chains/dev/paritydb/full/stats.txt");
			assert!(std::path::Path::exists(&stats_file));
		}

		run_command_and_kill(SIGINT).await;
		run_command_and_kill(SIGTERM).await;
	})
	.await;
}

#[tokio::test]
async fn running_two_nodes_with_the_same_ws_port_should_work() {
	common::run_with_timeout(Duration::from_secs(60 * 10), async move {
		let mut first_node = common::KillChildOnDrop(common::start_node());
		let mut second_node = common::KillChildOnDrop(common::start_node());

		common::wait_n_finalized_blocks(3, &common::ws_url_from_port(common::START_NODE_RPC_PORT))
			.await;

		first_node.assert_still_running();
		second_node.assert_still_running();

		first_node.stop();
		second_node.stop();
	})
	.await;
}
