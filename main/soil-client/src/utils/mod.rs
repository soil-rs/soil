// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utilities Primitives for Substrate
//!
//! This crate provides `mpsc::tracing_unbounded` function that returns wrapper types to
//! `async_channel::Sender<T>` and `async_channel::Receiver<T>`, which register every
//! `send`/`received`/`dropped` action happened on the channel.
//!
//! Also this wrapper creates and registers a prometheus vector with name `unbounded_channel_len`
//! and labels:
//!
//! | Label        | Description                                   |
//! | ------------ | --------------------------------------------- |
//! | entity       | Name of channel passed to `tracing_unbounded` |
//! | action       | One of `send`/`received`/`dropped`            |

pub mod id_sequence;
pub mod metrics;
pub mod mpsc;
pub mod notification;
pub mod pubsub;
pub mod status_sinks;
