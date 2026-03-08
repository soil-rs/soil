// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[rustversion::attr(not(stable), ignore)]
#[cfg(not(feature = "disable-ui-tests"))]
#[test]
fn split_ui() {
	// Only run the ui tests when `RUN_UI_TESTS` is set.
	if std::env::var("RUN_UI_TESTS").is_err() {
		return;
	}

	// As trybuild is using `cargo check`, we don't need the real WASM binaries.
	std::env::set_var("SKIP_WASM_BUILD", "1");

	// Deny all warnings since we emit warnings as part of a Pallet's UI.
	std::env::set_var("CARGO_ENCODED_RUSTFLAGS", "--deny=warnings");

	let t = trybuild::TestCases::new();
	t.compile_fail("tests/split_ui/*.rs");
	t.pass("tests/split_ui/pass/*.rs");
}
