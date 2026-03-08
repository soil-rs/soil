// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(unix)]

use assert_cmd::cargo::cargo_bin;
use std::{
	process::{Command, Stdio},
	time::Duration,
};

use soil_test_staging_node_cli_test_utils as common;

#[allow(dead_code)]
// Apparently `#[ignore]` doesn't actually work to disable this one.
//#[tokio::test]
async fn temp_base_path_works() {
	common::run_with_timeout(Duration::from_secs(60 * 10), async move {
		let mut cmd = Command::new(cargo_bin("soil-test-staging-node"));
		let mut child = common::KillChildOnDrop(
			cmd.args(&["--dev", "--tmp", "--no-hardware-benchmarks"])
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.unwrap(),
		);

		let stderr = child.stderr.take().unwrap();
		let node_info = common::extract_info_from_output(stderr).0;

		// Let it produce some blocks.
		common::wait_n_finalized_blocks(3, &node_info.ws_url).await;

		// Ensure the db path exists while the node is running
		assert!(node_info.db_path.exists());

		child.assert_still_running();

		// Stop the process
		child.stop();

		if node_info.db_path.exists() {
			panic!("Database path `{}` wasn't deleted!", node_info.db_path.display());
		}
	})
	.await;
}
