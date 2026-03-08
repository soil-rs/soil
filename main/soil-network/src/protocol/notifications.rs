// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Implementation of libp2p's `NetworkBehaviour` trait that establishes communications and opens
//! notifications substreams.

pub use self::{
	behaviour::{Notifications, NotificationsOut, ProtocolConfig},
	handler::{NotificationsSink, Ready},
	service::{notification_service, ProtocolHandlePair},
};

pub(crate) use self::service::ProtocolHandle;

mod behaviour;
mod handler;
mod service;
mod tests;
mod upgrade;
