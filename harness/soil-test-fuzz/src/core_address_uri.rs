// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Fuzzing for `AddressUri::parse`.
//!
//! # Running
//!
//! Run with `cargo hfuzz run core_address_uri`.
//!
//! # Debugging a panic
//!
//! Once a panic is found, it can be debugged with
//! `cargo hfuzz run-debug core_address_uri hfuzz_workspace/core_address_uri/*.fuzz`.

use honggfuzz::fuzz;
use regex::Regex;
use std::sync::LazyLock;
use subsoil::core::crypto::AddressUri;

static SECRET_PHRASE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"^(?P<phrase>[a-zA-Z0-9 ]+)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$")
		.expect("constructed from known-good static value; qed")
});

fn main() {
	loop {
		fuzz!(|data: &[u8]| {
			let Ok(input) = std::str::from_utf8(data) else {
				return;
			};

			let regex_result = SECRET_PHRASE_REGEX.captures(input);
			let manual_result = AddressUri::parse(input);
			assert_eq!(regex_result.is_some(), manual_result.is_ok());
			if manual_result.is_err() {
				let _ = format!("{}", manual_result.as_ref().err().unwrap());
			}
			if let (Some(regex_result), Ok(manual_result)) = (regex_result, manual_result) {
				assert_eq!(regex_result.name("phrase").map(|p| p.as_str()), manual_result.phrase);

				let manual_paths = manual_result
					.paths
					.iter()
					.map(|s| "/".to_string() + s)
					.collect::<Vec<_>>()
					.join("");

				assert_eq!(regex_result.name("path").unwrap().as_str().to_string(), manual_paths);
				assert_eq!(
					regex_result.name("password").map(|pass| pass.as_str()),
					manual_result.pass
				);
			}
		});
	}
}
