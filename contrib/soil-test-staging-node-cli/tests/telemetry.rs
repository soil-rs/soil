// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use assert_cmd::cargo::cargo_bin;
use nix::{
	sys::signal::{kill, Signal::SIGINT},
	unistd::Pid,
};
use std::{process, time::Duration};
use tokio::sync::oneshot;

use crate::common::KillChildOnDrop;

use soil_test_staging_node_cli_test_utils as common;
pub mod websocket_server;

#[tokio::test]
async fn telemetry_works() {
	common::run_with_timeout(Duration::from_secs(60 * 10), async move {
		let config = websocket_server::Config {
			capacity: 1,
			max_frame_size: 1048 * 1024,
			send_buffer_len: 32,
			bind_address: "127.0.0.1:0".parse().unwrap(),
		};
		let mut server = websocket_server::WsServer::new(config).await.unwrap();

		let addr = server.local_addr().unwrap();
		let (ready_tx, ready_rx) = oneshot::channel();

		let server_task = tokio::spawn(async move {
			let mut ready_tx = Some(ready_tx);

			loop {
				use websocket_server::Event;
				match server.next_event().await {
					// New connection on the listener.
					Event::ConnectionOpen { address } => {
						println!("New connection from {:?}", address);
						server.accept();
					},

					// Received a message from a connection.
					Event::BinaryFrame { message, .. } => {
						let json: serde_json::Value = serde_json::from_slice(&message).unwrap();
						let object =
							json.as_object().unwrap().get("payload").unwrap().as_object().unwrap();
						if matches!(object.get("best"), Some(serde_json::Value::String(_))) {
							if let Some(ready_tx) = ready_tx.take() {
								let _ = ready_tx.send(());
							}
						}
					},

					Event::TextFrame { .. } => {
						panic!("Got a TextFrame over the socket, this is a bug")
					},

					// Connection has been closed.
					Event::ConnectionError { .. } => {},
				}
			}
		});

		let mut substrate = process::Command::new(cargo_bin("soil-test-staging-node"));

		let mut substrate = KillChildOnDrop(
			substrate
				.args(&["--dev", "--tmp", "--telemetry-url"])
				.arg(format!("ws://{} 10", addr))
				.arg("--no-hardware-benchmarks")
				.stdout(process::Stdio::piped())
				.stderr(process::Stdio::piped())
				.stdin(process::Stdio::null())
				.spawn()
				.unwrap(),
		);

		ready_rx.await.expect("telemetry payload was never observed");

		substrate.assert_still_running();

		// This test only asserts that telemetry connects and emits payloads.
		// The exact exit status after SIGINT is not the behavior under test.
		kill(Pid::from_raw(substrate.id().try_into().unwrap()), SIGINT).unwrap();
		let _ = substrate.wait();
		server_task.abort();
	})
	.await;
}
