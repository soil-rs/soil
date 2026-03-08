// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! [`PendingResponses`] is responsible for keeping track of pending responses and
//! polling them. [`Stream`] implemented by [`PendingResponses`] never terminates.

use crate::{sync::strategy::StrategyKey, LOG_TARGET};
use futures::{
	channel::oneshot,
	future::BoxFuture,
	stream::{BoxStream, FusedStream, Stream},
	FutureExt, StreamExt,
};
use log::error;
use std::any::Any;

use soil_network::types::PeerId;
use soil_network::{request_responses::RequestFailure, types::ProtocolName};
use std::task::{Context, Poll, Waker};
use tokio_stream::StreamMap;

/// Response result.
type ResponseResult =
	Result<Result<(Box<dyn Any + Send>, ProtocolName), RequestFailure>, oneshot::Canceled>;

/// A future yielding [`ResponseResult`].
pub(crate) type ResponseFuture = BoxFuture<'static, ResponseResult>;

/// An event we receive once a pending response future resolves.
pub(crate) struct ResponseEvent {
	pub peer_id: PeerId,
	pub key: StrategyKey,
	pub response: ResponseResult,
}

/// Stream taking care of polling pending responses.
pub(crate) struct PendingResponses {
	/// Pending responses
	pending_responses: StreamMap<(PeerId, StrategyKey), BoxStream<'static, ResponseResult>>,
	/// Waker to implement never terminating stream
	waker: Option<Waker>,
}

impl PendingResponses {
	pub fn new() -> Self {
		Self { pending_responses: StreamMap::new(), waker: None }
	}

	pub fn insert(&mut self, peer_id: PeerId, key: StrategyKey, response_future: ResponseFuture) {
		if self
			.pending_responses
			.insert((peer_id, key), Box::pin(response_future.into_stream()))
			.is_some()
		{
			error!(
				target: LOG_TARGET,
				"Discarded pending response from peer {peer_id}, strategy key: {key:?}.",
			);
			debug_assert!(false);
		}

		if let Some(waker) = self.waker.take() {
			waker.wake();
		}
	}

	pub fn remove(&mut self, peer_id: PeerId, key: StrategyKey) -> bool {
		self.pending_responses.remove(&(peer_id, key)).is_some()
	}

	pub fn remove_all(&mut self, peer_id: &PeerId) {
		let to_remove = self
			.pending_responses
			.keys()
			.filter(|(peer, _key)| peer == peer_id)
			.cloned()
			.collect::<Vec<_>>();
		to_remove.iter().for_each(|k| {
			self.pending_responses.remove(k);
		});
	}

	pub fn len(&self) -> usize {
		self.pending_responses.len()
	}
}

impl Stream for PendingResponses {
	type Item = ResponseEvent;

	fn poll_next(
		mut self: std::pin::Pin<&mut Self>,
		cx: &mut Context<'_>,
	) -> Poll<Option<Self::Item>> {
		match self.pending_responses.poll_next_unpin(cx) {
			Poll::Ready(Some(((peer_id, key), response))) => {
				// We need to manually remove the stream, because `StreamMap` doesn't know yet that
				// it's going to yield `None`, so may not remove it before the next request is made
				// to the same peer.
				self.pending_responses.remove(&(peer_id, key));

				Poll::Ready(Some(ResponseEvent { peer_id, key, response }))
			},
			Poll::Ready(None) | Poll::Pending => {
				self.waker = Some(cx.waker().clone());

				Poll::Pending
			},
		}
	}
}

// As [`PendingResponses`] never terminates, we can easily implement [`FusedStream`] for it.
impl FusedStream for PendingResponses {
	fn is_terminated(&self) -> bool {
		false
	}
}
