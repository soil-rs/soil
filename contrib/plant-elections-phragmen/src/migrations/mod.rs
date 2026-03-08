// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! All migrations of this pallet.

/// Migration to unreserve all pallet funds.
pub mod unlock_and_unreserve_all_funds;
/// Version 3.
pub mod v3;
/// Version 4.
pub mod v4;
/// Version 5.
pub mod v5;
