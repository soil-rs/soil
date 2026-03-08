// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate Node CLI

#![warn(missing_docs)]

use soil_test_staging_node_cli as node_cli;

fn main() -> soil_cli::Result<()> {
	node_cli::run()
}
