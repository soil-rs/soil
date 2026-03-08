// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate chain head API.
//!
//! # Note
//!
//! Methods are prefixed by `chainHead`.

#[cfg(test)]
pub mod test_utils;
#[cfg(test)]
mod tests;

pub mod api;
pub mod chain_head;
pub mod error;
pub mod event;

mod chain_head_follow;
mod chain_head_storage;
mod subscription;

pub use api::ChainHeadApiServer;
pub use chain_head::{ChainHead, ChainHeadConfig};
pub use event::{
	BestBlockChanged, ErrorEvent, Finalized, FollowEvent, Initialized, NewBlock, RuntimeEvent,
	RuntimeVersionEvent,
};

/// Follow event sender.
pub(crate) type FollowEventSender<Hash> = futures::channel::mpsc::Sender<FollowEvent<Hash>>;
/// Follow event receiver.
pub(crate) type FollowEventReceiver<Hash> = futures::channel::mpsc::Receiver<FollowEvent<Hash>>;
/// Follow event send error.
pub(crate) type FollowEventSendError = futures::channel::mpsc::SendError;
