// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg(unix)]

use assert_cmd::cargo::cargo_bin;
use soil_test_staging_node_cli as node_cli;
use nix::{
	sys::signal::{kill, Signal, Signal::SIGINT},
	unistd::Pid,
};
use soil_test_staging_node_primitives::{Hash, Header};
use regex::Regex;
use soil_rpc::{list::ListOrValue, number::NumberOrHex};
use std::{
	io::{BufRead, BufReader, Read},
	net::TcpListener,
	ops::{Deref, DerefMut},
	path::{Path, PathBuf},
	process::{self, Child, Command},
	sync::mpsc,
	thread,
	time::Duration,
};
use tokio::io::{AsyncBufReadExt, AsyncRead};

pub const START_NODE_RPC_PORT: u16 = 45789;

pub fn find_free_tcp_port() -> u16 {
	let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
	let port = listener.local_addr().unwrap().port();
	drop(listener);
	port
}

pub fn ws_url_from_port(port: u16) -> String {
	format!("ws://127.0.0.1:{port}")
}

/// Similar to [`crate::start_node`] spawns a node, but works in environments where the substrate
/// binary is not accessible with `cargo_bin("soil-test-staging-node")`, and allows customising the args
/// passed in.
///
/// Helpful if you need a Substrate dev node running in the background of a project external to
/// `substrate`.
///
/// The downside compared to using [`crate::start_node`] is that this method is blocking rather than
/// returning a [`Child`]. Therefore, you may want to call this method inside a new thread.
///
/// # Example
/// ```ignore
/// // Spawn a dev node.
/// let _ = std::thread::spawn(move || {
///     match common::start_node_inline(vec!["--dev", "--rpc-port=12345"]) {
///         Ok(_) => {}
///         Err(e) => {
///             panic!("Node exited with error: {}", e);
///         }
///     }
/// });
/// ```
pub fn start_node_inline(args: Vec<&str>) -> Result<(), soil_service::error::Error> {
	use soil_cli::SubstrateCli;

	// Prepend the args with some dummy value because the first arg is skipped.
	let cli_call = std::iter::once("node-template").chain(args);
	let cli = node_cli::Cli::from_iter(cli_call);
	let runner = cli.create_runner(&cli.run).unwrap();
	runner.run_node_until_exit(|config| async move { node_cli::service::new_full(config, cli) })
}

/// Starts a new Substrate node in development mode with a temporary chain.
///
/// This function creates a new Substrate node using the `substrate` binary.
/// It configures the node to run in development mode (`--dev`) with a temporary chain (`--tmp`),
/// sets the JSON-RPC port to 45789.
///
/// # Returns
///
/// A [`Child`] process representing the spawned Substrate node.
///
/// # Panics
///
/// This function will panic if the `substrate` binary is not found or if the node fails to start.
///
/// # Examples
///
/// ```ignore
/// use my_crate::start_node;
///
/// let child = start_node();
/// // Interact with the Substrate node using the WebSocket port 45789.
/// // When done, the node will be killed when the `child` is dropped.
/// ```
///
/// [`Child`]: std::process::Child
pub fn start_node() -> Child {
	let mut command = Command::new(cargo_bin("soil-test-staging-node"));
	command
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.arg("--dev")
		.arg("--tmp")
		.arg(format!("--rpc-port={}", START_NODE_RPC_PORT))
		.arg("--no-hardware-benchmarks");
	command.spawn().unwrap()
}

/// Builds the Substrate project using the provided arguments.
///
/// This function reads the CARGO_MANIFEST_DIR environment variable to find the root workspace
/// directory. It then runs the `cargo b` command in the root directory with the specified
/// arguments.
///
/// This can be useful for building the Substrate binary with a desired set of features prior
/// to using the binary in a CLI test.
///
/// # Arguments
///
/// * `args: &[&str]` - A slice of string references representing the arguments to pass to the
///   `cargo b` command.
///
/// # Panics
///
/// This function will panic if:
///
/// * The CARGO_MANIFEST_DIR environment variable is not set.
/// * The root workspace directory cannot be determined.
/// * The 'cargo b' command fails to execute.
/// * The 'cargo b' command returns a non-successful status.
///
/// # Examples
///
/// ```ignore
/// build_substrate(&["--features=try-runtime"]);
/// ```
pub fn build_substrate(args: &[&str]) {
	let is_release_build = !cfg!(build_profile = "debug");

	// Get the root workspace directory from the CARGO_MANIFEST_DIR environment variable
	let mut cmd = Command::new("cargo");

	cmd.arg("build").arg("-p=soil-test-staging-node-cli");

	if is_release_build {
		cmd.arg("--release");
	}

	let output = cmd
		.args(args)
		.output()
		.expect(format!("Failed to execute 'cargo b' with args {:?}'", args).as_str());

	if !output.status.success() {
		panic!(
			"Failed to execute 'cargo b' with args {:?}': \n{}",
			args,
			String::from_utf8_lossy(&output.stderr)
		);
	}
}

/// Takes a readable tokio stream (e.g. from a child process `ChildStderr` or `ChildStdout`) and
/// a `Regex` pattern, and checks each line against the given pattern as it is produced.
/// The function returns OK(()) as soon as a line matching the pattern is found, or an Err if
/// the stream ends without any lines matching the pattern.
///
/// # Arguments
///
/// * `child_stream` - An async tokio stream, e.g. from a child process `ChildStderr` or
///   `ChildStdout`.
/// * `re` - A `Regex` pattern to search for in the stream.
///
/// # Returns
///
/// * `Ok(())` if a line matching the pattern is found.
/// * `Err(String)` if the stream ends without any lines matching the pattern.
///
/// # Examples
///
/// ```ignore
/// use regex::Regex;
/// use tokio::process::Command;
/// use tokio::io::AsyncRead;
///
/// # async fn run() {
/// let child = Command::new("some-command").stderr(std::process::Stdio::piped()).spawn().unwrap();
/// let stderr = child.stderr.unwrap();
/// let re = Regex::new("error:").unwrap();
///
/// match wait_for_pattern_match_in_stream(stderr, re).await {
///     Ok(()) => println!("Error found in stderr"),
///     Err(e) => println!("Error: {}", e),
/// }
/// # }
/// ```
pub async fn wait_for_stream_pattern_match<R>(stream: R, re: Regex) -> Result<(), String>
where
	R: AsyncRead + Unpin,
{
	let mut stdio_reader = tokio::io::BufReader::new(stream).lines();
	while let Ok(Some(line)) = stdio_reader.next_line().await {
		match re.find(line.as_str()) {
			Some(_) => return Ok(()),
			None => (),
		}
	}
	Err(String::from("Stream closed without any lines matching the regex."))
}

/// Run the given `future` and panic if the `timeout` is hit.
pub async fn run_with_timeout(timeout: Duration, future: impl futures::Future<Output = ()>) {
	tokio::time::timeout(timeout, future).await.expect("Hit timeout");
}

/// Wait for at least n blocks to be finalized from a specified node
pub async fn wait_n_finalized_blocks(n: usize, url: &str) {
	use soil_rpc::client::{ws_client, ChainApi};

	let mut built_blocks = std::collections::HashSet::new();
	let block_duration = Duration::from_secs(2);
	let mut interval = tokio::time::interval(block_duration);
	let rpc = loop {
		match ws_client(url).await {
			Ok(rpc) => break rpc,
			Err(_) => {
				interval.tick().await;
			},
		}
	};

	loop {
		if let Ok(block) = ChainApi::<(), Hash, Header, ()>::finalized_head(&rpc).await {
			built_blocks.insert(block);
			if built_blocks.len() > n {
				break;
			}
		};
		interval.tick().await;
	}
}

/// Run the node for a while (3 blocks)
pub async fn run_node_for_a_while(base_path: &Path, args: &[&str]) {
	run_with_timeout(Duration::from_secs(60 * 10), async move {
		let rpc_port = find_free_tcp_port();
		let cmd = Command::new(cargo_bin("soil-test-staging-node"))
			.stdout(process::Stdio::null())
			.stderr(process::Stdio::null())
			.args(args)
			.arg("--rpc-port")
			.arg(rpc_port.to_string())
			.arg("-d")
			.arg(base_path)
			.spawn()
			.unwrap();

		let mut child = KillChildOnDrop(cmd);

		// Let it produce some blocks.
		wait_n_finalized_blocks(3, &ws_url_from_port(rpc_port)).await;

		child.assert_still_running();

		// Stop the process
		child.stop();
	})
	.await
}

pub async fn block_hash(block_number: u64, url: &str) -> Result<Hash, String> {
	use soil_rpc::client::{ws_client, ChainApi};

	let rpc = ws_client(url).await.unwrap();

	let result = ChainApi::<(), Hash, Header, ()>::block_hash(
		&rpc,
		Some(ListOrValue::Value(NumberOrHex::Number(block_number))),
	)
	.await
	.map_err(|_| "Couldn't get block hash".to_string())?;

	match result {
		ListOrValue::Value(maybe_block_hash) if maybe_block_hash.is_some() => {
			Ok(maybe_block_hash.unwrap())
		},
		_ => Err("Couldn't get block hash".to_string()),
	}
}

pub struct KillChildOnDrop(pub Child);

impl KillChildOnDrop {
	/// Stop the child and wait until it is finished.
	///
	/// Asserts if the exit status isn't success.
	pub fn stop(&mut self) {
		self.stop_with_signal(SIGINT);
	}

	/// Same as [`Self::stop`] but takes the `signal` that is sent to stop the child.
	pub fn stop_with_signal(&mut self, signal: Signal) {
		kill(Pid::from_raw(self.id().try_into().unwrap()), signal).unwrap();
		assert!(self.wait().unwrap().success());
	}

	/// Asserts that the child is still running.
	pub fn assert_still_running(&mut self) {
		assert!(self.try_wait().unwrap().is_none(), "the process should still be running");
	}
}

impl Drop for KillChildOnDrop {
	fn drop(&mut self) {
		let _ = self.0.kill();
	}
}

impl Deref for KillChildOnDrop {
	type Target = Child;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for KillChildOnDrop {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Information extracted from a running node.
pub struct NodeInfo {
	pub ws_url: String,
	pub db_path: PathBuf,
}

/// Extract [`NodeInfo`] from a running node by parsing its output.
///
/// Returns the [`NodeInfo`] and all the read data.
pub fn extract_info_from_output(read: impl Read + Send + 'static) -> (NodeInfo, String) {
	let (tx, rx) = mpsc::sync_channel(1);

	thread::spawn(move || {
		let re = Regex::new(r"Database: .+ at (\S+)").unwrap();
		let mut data = String::new();
		let mut ws_url = None;
		let mut db_path = None;
		let mut sent = false;

		for line in BufReader::new(read).lines() {
			let line = match line {
				Ok(line) => line,
				Err(error) => {
					let _ = tx.send(Err(format!(
						"failed to obtain next line while extracting node info: {error}\n{data}"
					)));
					return;
				},
			};

			data.push_str(&line);
			data.push('\n');

			if db_path.is_none() {
				db_path = re.captures(&line).map(|captures| PathBuf::from(&captures[1]));
			}

			if ws_url.is_none() {
				ws_url = line
					.split_once("Running JSON-RPC server: addr=")
					.map(|(_, after)| after.split_once(",").unwrap().0)
					.map(|sock_addr| format!("ws://{sock_addr}"));
			}

			if !sent {
				if let (Some(ws_url), Some(db_path)) = (ws_url.as_ref(), db_path.as_ref()) {
					let _ = tx.send(Ok((
						NodeInfo { ws_url: ws_url.clone(), db_path: db_path.clone() },
						data.clone(),
					)));
					sent = true;
				}
			}
		}

		if !sent {
			let _ = tx.send(Err(data));
		}
	});

	match rx.recv_timeout(Duration::from_secs(60)) {
		Ok(Ok(result)) => result,
		Ok(Err(data)) => {
			eprintln!("Observed node output:\n{}", data);
			panic!("We should get node info from process output");
		},
		Err(error) => panic!("Timed out waiting for node info from process output: {error}"),
	}
}
