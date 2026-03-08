// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate network possible errors.

use crate::{config::TransportConfig, types::ProtocolName};

use crate::types::{multiaddr::Multiaddr, PeerId};

use std::fmt;

/// Result type alias for the network.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the network.
#[derive(thiserror::Error)]
pub enum Error {
	/// Io error
	#[error(transparent)]
	Io(#[from] std::io::Error),

	/// Client error
	#[error(transparent)]
	Client(#[from] Box<soil_client::blockchain::Error>),
	/// The same bootnode (based on address) is registered with two different peer ids.
	#[error(
		"The same bootnode (`{address}`) is registered with two different peer ids: `{first_id}` and `{second_id}`"
	)]
	DuplicateBootnode {
		/// The address of the bootnode.
		address: Multiaddr,
		/// The first peer id that was found for the bootnode.
		first_id: PeerId,
		/// The second peer id that was found for the bootnode.
		second_id: PeerId,
	},
	/// Prometheus metrics error.
	#[error(transparent)]
	Prometheus(#[from] soil_prometheus::PrometheusError),
	/// The network addresses are invalid because they don't match the transport.
	#[error(
		"The following addresses are invalid because they don't match the transport: {addresses:?}"
	)]
	AddressesForAnotherTransport {
		/// Transport used.
		transport: TransportConfig,
		/// The invalid addresses.
		addresses: Vec<Multiaddr>,
	},
	/// The same request-response protocol has been registered multiple times.
	#[error("Request-response protocol registered multiple times: {protocol}")]
	DuplicateRequestResponseProtocol {
		/// Name of the protocol registered multiple times.
		protocol: ProtocolName,
	},
	/// Peer does not exist.
	#[error("Peer `{0}` does not exist.")]
	PeerDoesntExist(PeerId),
	/// Channel closed.
	#[error("Channel closed")]
	ChannelClosed,
	/// Connection closed.
	#[error("Connection closed")]
	ConnectionClosed,
	/// Litep2p error.
	#[error("Litep2p error: `{0}`")]
	Litep2p(litep2p::Error),
}

// Make `Debug` use the `Display` implementation.
impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}
