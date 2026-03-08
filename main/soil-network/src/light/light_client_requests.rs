// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Helpers for outgoing and incoming light client requests.

use crate::{
	config::ProtocolId, request_responses::IncomingRequest, NetworkBackend, MAX_RESPONSE_SIZE,
};
use subsoil::runtime::traits::Block;

use std::time::Duration;

/// For incoming light client requests.
pub mod handler;

/// Generate the light client protocol name from the genesis hash and fork id.
fn generate_protocol_name<Hash: AsRef<[u8]>>(genesis_hash: Hash, fork_id: Option<&str>) -> String {
	let genesis_hash = genesis_hash.as_ref();
	if let Some(fork_id) = fork_id {
		format!("/{}/{}/light/2", array_bytes::bytes2hex("", genesis_hash), fork_id)
	} else {
		format!("/{}/light/2", array_bytes::bytes2hex("", genesis_hash))
	}
}

/// Generate the legacy light client protocol name from chain specific protocol identifier.
fn generate_legacy_protocol_name(protocol_id: &ProtocolId) -> String {
	format!("/{}/light/2", protocol_id.as_ref())
}

/// Generates a `RequestResponseProtocolConfig` for the light client request protocol, refusing
/// incoming requests.
pub fn generate_protocol_config<
	Hash: AsRef<[u8]>,
	B: Block,
	N: NetworkBackend<B, <B as Block>::Hash>,
>(
	protocol_id: &ProtocolId,
	genesis_hash: Hash,
	fork_id: Option<&str>,
	inbound_queue: async_channel::Sender<IncomingRequest>,
) -> N::RequestResponseProtocolConfig {
	N::request_response_config(
		generate_protocol_name(genesis_hash, fork_id).into(),
		std::iter::once(generate_legacy_protocol_name(protocol_id).into()).collect(),
		1 * 1024 * 1024,
		MAX_RESPONSE_SIZE,
		Duration::from_secs(15),
		Some(inbound_queue),
	)
}
