// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use soil_network::{
	config::{
		FullNetworkConfiguration, IncomingRequest, MultiaddrWithPeerId, NetworkConfiguration,
		NonReservedPeerMode, NotificationHandshake, OutgoingResponse, Params, ProtocolId, Role,
		SetConfig,
	},
	service::traits::{NetworkService, NotificationEvent},
	IfDisconnected, NetworkBackend, NetworkRequest, NotificationMetrics, NotificationService,
	Roles,
};

use soil_network::common::sync::message::BlockAnnouncesHandshake;
use std::{sync::Arc, time::Duration};
use subsoil::runtime::traits::Zero;
use soil_test_node_runtime_client::runtime;
use tokio::sync::Mutex;

/// High level network backend (litep2p or libp2p) test client.
pub struct NetworkBackendClient {
	pub network_service: Arc<dyn NetworkService>,
	pub notification_service: Arc<Mutex<Box<dyn NotificationService>>>,
	pub receiver: async_channel::Receiver<IncomingRequest>,
}

/// Configure the network backend client for tests based on the given service.
///
/// This will setup:
/// - `/request-response/1` request response protocol with bounded channel of 32 requests
/// - `/block-announces/1` notification protocol
pub fn create_network_backend<N>() -> NetworkBackendClient
where
	N: NetworkBackend<runtime::Block, runtime::Hash>,
{
	let (tx, rx) = async_channel::bounded(32);
	let request_response_config = N::request_response_config(
		"/request-response/1".into(),
		vec![],
		1024,
		1024,
		Duration::from_secs(2),
		Some(tx),
	);

	let role = Role::Full;
	let net_conf = NetworkConfiguration::new_local();
	let mut network_config = FullNetworkConfiguration::new(&net_conf, None);
	network_config.add_request_response_protocol(request_response_config);
	let genesis_hash = runtime::Hash::zero();
	let (block_announce_config, notification_service) = N::notification_config(
		"/block-announces/1".into(),
		vec![],
		1024,
		Some(NotificationHandshake::new(BlockAnnouncesHandshake::<runtime::Block>::build(
			Roles::from(&Role::Full),
			Zero::zero(),
			genesis_hash,
			genesis_hash,
		))),
		SetConfig {
			in_peers: 1,
			out_peers: 1,
			reserved_nodes: vec![],
			non_reserved_mode: NonReservedPeerMode::Accept,
		},
		NotificationMetrics::new(None),
		network_config.peer_store_handle(),
	);
	let worker = N::new(Params::<runtime::Block, runtime::Hash, N> {
		block_announce_config,
		role,
		executor: Box::new(|f| {
			tokio::spawn(f);
		}),
		genesis_hash: runtime::Hash::zero(),
		network_config,
		protocol_id: ProtocolId::from("test"),
		fork_id: None,
		metrics_registry: None,
		bitswap_config: None,
		notification_metrics: NotificationMetrics::new(None),
	})
	.unwrap();
	let network_service = worker.network_service();

	// Run the worker in the backend.
	tokio::spawn(worker.run());

	NetworkBackendClient {
		network_service,
		notification_service: Arc::new(Mutex::new(notification_service)),
		receiver: rx,
	}
}

/// Connect two backends together and submit one request with `IfDisconnected::TryConnect` option
/// expecting the left backend to dial the right one.
pub async fn connect_backends(left: &NetworkBackendClient, right: &NetworkBackendClient) {
	let right_peer_id = right.network_service.local_peer_id();

	// Ensure the right backend responds to a first request
	let rx = right.receiver.clone();
	tokio::spawn(async move {
		let request = rx.recv().await.expect("Left backend should receive a request");
		assert_eq!(request.payload, vec![1, 2, 3]);
		request
			.pending_response
			.send(OutgoingResponse {
				result: Ok(vec![4, 5, 6]),
				reputation_changes: vec![],
				sent_feedback: None,
			})
			.expect("Left backend should send a response");
	});

	// Connect the two backends
	while left.network_service.listen_addresses().is_empty() {
		tokio::time::sleep(Duration::from_millis(10)).await;
	}
	while right.network_service.listen_addresses().is_empty() {
		tokio::time::sleep(Duration::from_millis(10)).await;
	}
	let right_listen_address = right
		.network_service
		.listen_addresses()
		.first()
		.expect("qed; non empty")
		.clone();

	left.network_service
		.add_known_address(right_peer_id, right_listen_address.clone().into());

	let result = left
		.network_service
		.request(
			right_peer_id,
			"/request-response/1".into(),
			vec![1, 2, 3],
			None,
			IfDisconnected::TryConnect,
		)
		.await
		.expect("Left backend should send a request");
	assert_eq!(result.0, vec![4, 5, 6]);
	assert_eq!(result.1, "/request-response/1".into());
}

/// Ensure connectivity on the notification protocol level.
pub async fn connect_notifications(left: &NetworkBackendClient, right: &NetworkBackendClient) {
	let right_peer_id = right.network_service.local_peer_id();

	while left.network_service.listen_addresses().is_empty() {
		tokio::time::sleep(Duration::from_millis(10)).await;
	}
	while right.network_service.listen_addresses().is_empty() {
		tokio::time::sleep(Duration::from_millis(10)).await;
	}

	let right_listen_address = right
		.network_service
		.listen_addresses()
		.first()
		.expect("qed; non empty")
		.clone();

	left.network_service
		.add_reserved_peer(MultiaddrWithPeerId {
			multiaddr: right_listen_address.into(),
			peer_id: right_peer_id,
		})
		.unwrap();

	let mut notifications_left = left.notification_service.lock().await;
	let mut notifications_right = right.notification_service.lock().await;
	let mut opened = 0;
	loop {
		tokio::select! {
			Some(event) = notifications_left.next_event() => {
				match event {
					NotificationEvent::NotificationStreamOpened { .. } => {
						opened += 1;
						if opened >= 2 {
							break;
						}
					},
					NotificationEvent::ValidateInboundSubstream { result_tx, .. } => {
						result_tx.send(soil_network::service::traits::ValidationResult::Accept).unwrap();
					},
					_ => {},
				};
			},
			Some(event) = notifications_right.next_event() => {
				match event {
					NotificationEvent::ValidateInboundSubstream { result_tx, .. } => {
						result_tx.send(soil_network::service::traits::ValidationResult::Accept).unwrap();
					},
					NotificationEvent::NotificationStreamOpened { .. } => {
						opened += 1;
						if opened >= 2 {
							break;
						}
					},
					_ => {}
				}
			},
		}
	}
}
