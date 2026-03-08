// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use tempfile::tempdir;

use soil_test_staging_node_cli_test_utils as common;

#[tokio::test]
#[cfg(unix)]
async fn remember_state_pruning_works() {
	let base_path = tempdir().expect("could not create a temp dir");

	// First run with `--state-pruning=archive`.
	common::run_node_for_a_while(
		base_path.path(),
		&["--dev", "--state-pruning=archive", "--no-hardware-benchmarks"],
	)
	.await;

	// Then run again without specifying the state pruning.
	// This should load state pruning settings from the db.
	common::run_node_for_a_while(base_path.path(), &["--dev", "--no-hardware-benchmarks"]).await;
}
