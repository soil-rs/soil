// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate RPC interfaces.
//!
//! A collection of RPC methods and subscriptions supported by all substrate clients.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod error;
#[cfg(feature = "std")]
mod policy;

#[cfg(feature = "std")]
pub use policy::{check_if_safe, DenyUnsafe, UnsafeRpcError};

#[cfg(feature = "std")]
pub mod author;
#[cfg(feature = "std")]
pub mod chain;
#[cfg(feature = "std")]
pub mod child_state;
#[cfg(feature = "std")]
pub mod dev;
#[cfg(feature = "std")]
pub mod mixnet;
#[cfg(feature = "std")]
pub mod offchain;
#[cfg(feature = "std")]
pub mod state;
#[cfg(feature = "std")]
pub mod statement;
#[cfg(feature = "std")]
pub mod system;
