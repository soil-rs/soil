// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use std::{collections::HashSet, fmt::Debug};

use super::ServicetoWorkerMsg;

use futures::{
	channel::{mpsc, oneshot},
	SinkExt,
};

use crate::AuthorityId;
use soil_network::types::PeerId;
use soil_network::Multiaddr;

/// Service to interact with the [`super::Worker`].
#[derive(Clone)]
pub struct Service {
	to_worker: mpsc::Sender<ServicetoWorkerMsg>,
}

impl Debug for Service {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("AuthorityDiscoveryService").finish()
	}
}

/// A [`Service`] allows to interact with a [`super::Worker`], e.g. by querying the
/// [`super::Worker`]'s local address cache for a given [`AuthorityId`].
impl Service {
	pub(crate) fn new(to_worker: mpsc::Sender<ServicetoWorkerMsg>) -> Self {
		Self { to_worker }
	}

	/// Get the addresses for the given [`AuthorityId`] from the local address
	/// cache.
	///
	/// Returns `None` if no entry was present or connection to the
	/// [`super::Worker`] failed.
	///
	/// Note: [`Multiaddr`]s returned always include a [`PeerId`] via a
	/// [`soil_network::types::multiaddr::Protocol::P2p`] component. Equality of
	/// [`PeerId`]s across [`Multiaddr`]s returned by a single call is not
	/// enforced today, given that there are still authorities out there
	/// publishing the addresses of their sentry nodes on the DHT. In the future
	/// this guarantee can be provided.
	pub async fn get_addresses_by_authority_id(
		&mut self,
		authority: AuthorityId,
	) -> Option<HashSet<Multiaddr>> {
		let (tx, rx) = oneshot::channel();

		self.to_worker
			.send(ServicetoWorkerMsg::GetAddressesByAuthorityId(authority, tx))
			.await
			.ok()?;

		rx.await.ok().flatten()
	}

	/// Get the [`AuthorityId`] for the given [`PeerId`] from the local address
	/// cache.
	///
	/// Returns `None` if no entry was present or connection to the
	/// [`super::Worker`] failed.
	pub async fn get_authority_ids_by_peer_id(
		&mut self,
		peer_id: PeerId,
	) -> Option<HashSet<AuthorityId>> {
		let (tx, rx) = oneshot::channel();

		self.to_worker
			.send(ServicetoWorkerMsg::GetAuthorityIdsByPeerId(peer_id, tx))
			.await
			.ok()?;

		rx.await.ok().flatten()
	}
}
