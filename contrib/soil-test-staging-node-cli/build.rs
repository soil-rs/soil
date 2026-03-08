// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

fn main() {
	#[cfg(feature = "cli")]
	cli::main();
}

#[cfg(feature = "cli")]
mod cli {
	include!("src/cli.rs");

	use clap::{CommandFactory, ValueEnum};
	use clap_complete::{generate_to, Shell};
	use std::{env, fs, path::Path};
	use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

	pub fn main() {
		build_shell_completion();
		generate_cargo_keys();

		rerun_if_git_head_changed();
	}

	/// Build shell completion scripts for all known shells.
	fn build_shell_completion() {
		for shell in Shell::value_variants() {
			build_completion(shell);
		}
	}

	/// Build the shell auto-completion for a given Shell.
	fn build_completion(shell: &Shell) {
		let outdir = match env::var_os("OUT_DIR") {
			None => return,
			Some(dir) => dir,
		};
		let path = Path::new(&outdir)
			.parent()
			.unwrap()
			.parent()
			.unwrap()
			.parent()
			.unwrap()
			.join("completion-scripts");

		fs::create_dir(&path).ok();

		let _ = generate_to(*shell, &mut Cli::command(), "soil-test-staging-node", &path);
	}
}
