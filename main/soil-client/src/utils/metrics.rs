// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Metering primitives and globals

use prometheus::{
	core::{AtomicU64, GenericCounter, GenericGauge},
	Error as PrometheusError, Registry,
};
use std::sync::LazyLock;

use prometheus::{
	core::{GenericCounterVec, GenericGaugeVec},
	Opts,
};

pub static TOKIO_THREADS_TOTAL: LazyLock<GenericCounter<AtomicU64>> = LazyLock::new(|| {
	GenericCounter::new("substrate_tokio_threads_total", "Total number of threads created")
		.expect("Creating of statics doesn't fail. qed")
});

pub static TOKIO_THREADS_ALIVE: LazyLock<GenericGauge<AtomicU64>> = LazyLock::new(|| {
	GenericGauge::new("substrate_tokio_threads_alive", "Number of threads alive right now")
		.expect("Creating of statics doesn't fail. qed")
});

pub static UNBOUNDED_CHANNELS_COUNTER: LazyLock<GenericCounterVec<AtomicU64>> =
	LazyLock::new(|| {
		GenericCounterVec::new(
			Opts::new(
				"substrate_unbounded_channel_len",
				"Items sent/received/dropped on each mpsc::unbounded instance",
			),
			&["entity", "action"], // name of channel, send|received|dropped
		)
		.expect("Creating of statics doesn't fail. qed")
	});

pub static UNBOUNDED_CHANNELS_SIZE: LazyLock<GenericGaugeVec<AtomicU64>> = LazyLock::new(|| {
	GenericGaugeVec::new(
		Opts::new(
			"substrate_unbounded_channel_size",
			"Size (number of messages to be processed) of each mpsc::unbounded instance",
		),
		&["entity"], // name of channel
	)
	.expect("Creating of statics doesn't fail. qed")
});

pub static SENT_LABEL: &'static str = "send";
pub static RECEIVED_LABEL: &'static str = "received";
pub static DROPPED_LABEL: &'static str = "dropped";

/// Register the statics to report to registry
pub fn register_globals(registry: &Registry) -> Result<(), PrometheusError> {
	registry.register(Box::new(TOKIO_THREADS_ALIVE.clone()))?;
	registry.register(Box::new(TOKIO_THREADS_TOTAL.clone()))?;
	registry.register(Box::new(UNBOUNDED_CHANNELS_COUNTER.clone()))?;
	registry.register(Box::new(UNBOUNDED_CHANNELS_SIZE.clone()))?;

	Ok(())
}
