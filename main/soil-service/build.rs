// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

fn main() {
	let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is always set in build scripts; qed");
	let out_dir = std::path::PathBuf::from(out_dir);
	let target_os = std::env::var("CARGO_CFG_TARGET_OS")
		.expect("CARGO_CFG_TARGET_OS is always set in build scripts; qed");
	let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
		.expect("CARGO_CFG_TARGET_ARCH is always set in build scripts; qed");
	let target_env = std::env::var("CARGO_CFG_TARGET_ENV")
		.expect("CARGO_CFG_TARGET_ENV is always set in build scripts; qed");
	std::fs::write(out_dir.join("target_os.txt"), target_os).unwrap();
	std::fs::write(out_dir.join("target_arch.txt"), target_arch).unwrap();
	std::fs::write(out_dir.join("target_env.txt"), target_env).unwrap();
}
