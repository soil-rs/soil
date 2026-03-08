// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate blockchain traits and primitives.

mod backend;
mod error;
mod header_metadata;

pub use backend::*;
pub use error::*;
pub use header_metadata::*;

const LOG_TARGET: &str = "db::blockchain";
