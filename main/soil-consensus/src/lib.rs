// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Collection of common engine-support consensus implementations.

pub mod epochs;
pub mod slots;

mod longest_chain;
pub mod shared_data;

pub use longest_chain::LongestChain;
