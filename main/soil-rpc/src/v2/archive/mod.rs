// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate archive API.
//!
//! # Note
//!
//! Methods are prefixed by `archive`.

#[cfg(test)]
mod tests;

mod archive_storage;
mod types;

pub mod api;
pub mod archive;
pub mod error;

pub use api::ArchiveApiServer;
pub use archive::Archive;
pub use types::{MethodResult, MethodResultErr, MethodResultOk};
