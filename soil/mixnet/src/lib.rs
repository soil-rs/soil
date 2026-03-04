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

//! Substrate mixnet types and runtime interface.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod runtime_api;
pub mod types;

// std-only modules (former sc-mixnet)
#[cfg(feature = "std")]
mod api;
#[cfg(feature = "std")]
mod config;
#[cfg(feature = "std")]
mod error;
#[cfg(feature = "std")]
mod extrinsic_queue;
#[cfg(feature = "std")]
mod maybe_inf_delay;
#[cfg(feature = "std")]
mod packet_dispatcher;
#[cfg(feature = "std")]
mod peer_id;
#[cfg(feature = "std")]
mod protocol;
#[cfg(feature = "std")]
mod request;
#[cfg(feature = "std")]
mod run;
#[cfg(feature = "std")]
mod sync_with_runtime;

#[cfg(feature = "std")]
pub use self::{
	api::{Api, ApiBackend},
	config::{Config, CoreConfig, SubstrateConfig},
	error::{Error, RemoteErr},
	protocol::{peers_set_config, protocol_name},
	run::run,
};
#[cfg(feature = "std")]
pub use mixnet::core::{KxSecret, PostErr, TopologyErr};
