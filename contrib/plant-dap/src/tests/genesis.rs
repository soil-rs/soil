// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Genesis tests for the DAP pallet.

use crate::mock::*;

type DapPallet = crate::Pallet<Test>;

#[test]
fn genesis_creates_buffer_account() {
	new_test_ext(true).execute_with(|| {
		let buffer = DapPallet::buffer_account();
		// Buffer account should exist after genesis (created via inc_providers)
		assert!(System::account_exists(&buffer));
	});
}
