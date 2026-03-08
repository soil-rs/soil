// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! All migrations of this pallet.

/// Migration to unlock and unreserve all pallet funds.
pub mod unlock_and_unreserve_all_funds;

/// V1 storage migrations for the preimage pallet.
pub mod v1;
