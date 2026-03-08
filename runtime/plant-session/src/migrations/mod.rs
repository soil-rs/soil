// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// Version 1.
///
/// In version 0 session historical pallet uses `Session` for storage module prefix.
/// In version 1 it uses its name as configured in `construct_runtime`.
/// This migration moves session historical pallet storages from old prefix to new prefix.
#[cfg(feature = "historical")]
pub mod historical;
pub mod v1;
