// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Types for working with tracing data in the RPC layer.

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

/// Container for all related spans and events for the block being traced.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockTrace {
	/// Hash of the block being traced
	pub block_hash: String,
	/// Parent hash
	pub parent_hash: String,
	/// Module targets that were recorded by the tracing subscriber.
	/// Empty string means record all targets.
	pub tracing_targets: String,
	/// Storage key targets used to filter out events that do not have one of the storage keys.
	/// Empty string means do not filter out any events.
	pub storage_keys: String,
	/// Method targets used to filter out events that do not have one of the event method.
	/// Empty string means do not filter out any events.
	pub methods: String,
	/// Vec of tracing spans
	pub spans: Vec<Span>,
	/// Vec of tracing events
	pub events: Vec<Event>,
}

/// Represents a tracing event, complete with recorded data.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
	/// Event target
	pub target: String,
	/// Associated data
	pub data: Data,
	/// Parent id, if it exists
	pub parent_id: Option<u64>,
}

/// Represents a single instance of a tracing span.
///
/// Exiting a span does not imply that the span will not be re-entered.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Span {
	/// id for this span
	pub id: u64,
	/// id of the parent span, if any
	pub parent_id: Option<u64>,
	/// Name of this span
	pub name: String,
	/// Target, typically module
	pub target: String,
	/// Indicates if the span is from wasm
	pub wasm: bool,
}

/// Holds associated values for a tracing span.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	/// HashMap of `String` values recorded while tracing
	pub string_values: FxHashMap<String, String>,
}

/// Error response for the `state_traceBlock` RPC.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TraceError {
	/// Error message
	pub error: String,
}

/// Response for the `state_traceBlock` RPC.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TraceBlockResponse {
	/// Error block tracing response
	TraceError(TraceError),
	/// Successful block tracing response
	BlockTrace(BlockTrace),
}
