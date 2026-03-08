// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(unix)]

use assert_cmd::cargo::cargo_bin;
use std::process::Command;
use tempfile::tempdir;

use soil_test_staging_node_cli_test_utils as common;

#[tokio::test]
async fn check_block_works() {
	let base_path = tempdir().expect("could not create a temp dir");

	common::run_node_for_a_while(base_path.path(), &["--dev", "--no-hardware-benchmarks"]).await;

	let status = Command::new(cargo_bin("soil-test-staging-node"))
		.args(&["check-block", "--dev", "-d"])
		.arg(base_path.path())
		.arg("1")
		.status()
		.unwrap();
	assert!(status.success());
}
