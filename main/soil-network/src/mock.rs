// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mocked components for tests.

use crate::{
	peer_store::{PeerStoreProvider, ProtocolHandle},
	ReputationChange,
};

use crate::common::role::ObservedRole;
use crate::types::PeerId;

use std::{collections::HashSet, sync::Arc};

/// No-op `PeerStore`.
#[derive(Debug)]
pub struct MockPeerStore {}

impl PeerStoreProvider for MockPeerStore {
	fn is_banned(&self, _peer_id: &PeerId) -> bool {
		// Make sure that the peer is not banned.
		false
	}

	fn register_protocol(&self, _protocol_handle: Arc<dyn ProtocolHandle>) {
		// Make sure not to fail.
	}

	fn report_disconnect(&self, _peer_id: PeerId) {
		// Make sure not to fail.
	}

	fn report_peer(&self, _peer_id: PeerId, _change: ReputationChange) {
		// Make sure not to fail.
	}

	fn peer_reputation(&self, _peer_id: &PeerId) -> i32 {
		// Make sure that the peer is not banned.
		0
	}

	fn peer_role(&self, _peer_id: &PeerId) -> Option<ObservedRole> {
		None
	}

	fn set_peer_role(&self, _peer_id: &PeerId, _role: ObservedRole) {
		unimplemented!();
	}

	fn outgoing_candidates(&self, _count: usize, _ignored: HashSet<PeerId>) -> Vec<PeerId> {
		unimplemented!()
	}

	fn add_known_peer(&self, _peer_id: PeerId) {
		unimplemented!()
	}
}
