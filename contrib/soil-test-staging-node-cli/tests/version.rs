// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use assert_cmd::cargo::cargo_bin;
use regex::Regex;
use std::process::Command;

fn expected_regex() -> Regex {
	Regex::new(r"^soil-test-staging-node (.+)-([a-f\d]+)$").unwrap()
}

#[test]
fn version_is_full() {
	let expected = expected_regex();
	let output = Command::new(cargo_bin("soil-test-staging-node")).args(&["--version"]).output().unwrap();

	assert!(output.status.success(), "command returned with non-success exit code");

	let output = dbg!(String::from_utf8_lossy(&output.stdout).trim().to_owned());
	let captures = expected.captures(output.as_str()).expect("could not parse version in output");

	assert_eq!(&captures[1], env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_regex_matches_properly() {
	let expected = expected_regex();

	let captures = expected.captures("soil-test-staging-node 2.0.0-da487d19d").unwrap();
	assert_eq!(&captures[1], "2.0.0");
	assert_eq!(&captures[2], "da487d19d");

	let captures = expected.captures("soil-test-staging-node 2.0.0-alpha.5-da487d19d").unwrap();
	assert_eq!(&captures[1], "2.0.0-alpha.5");
	assert_eq!(&captures[2], "da487d19d");
}
