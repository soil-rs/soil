// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use futures::StreamExt;

#[derive(Clone)]
pub struct DummyTracingKey;
impl TracingKeyStr for DummyTracingKey {
	const TRACING_KEY: &'static str = "test_notification_stream";
}

type StringStream = NotificationStream<String, DummyTracingKey>;

#[test]
fn notification_channel_simple() {
	let (sender, stream) = StringStream::channel();

	let test_payload = String::from("test payload");
	let closure_payload = test_payload.clone();

	// Create a future to receive a single notification
	// from the stream and verify its payload.
	let future = stream.subscribe(100_000).take(1).for_each(move |payload| {
		let test_payload = closure_payload.clone();
		async move {
			assert_eq!(payload, test_payload);
		}
	});

	// Send notification.
	let r: std::result::Result<(), ()> = sender.notify(|| Ok(test_payload));
	r.unwrap();

	// Run receiver future.
	tokio_test::block_on(future);
}
