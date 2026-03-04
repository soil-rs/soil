// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod id_sequence;
#[cfg(feature = "std")]
pub mod metrics;
#[cfg(feature = "std")]
pub mod mpsc;
#[cfg(feature = "std")]
pub mod notification;
#[cfg(feature = "std")]
pub mod pubsub;
#[cfg(feature = "std")]
pub mod status_sinks;
