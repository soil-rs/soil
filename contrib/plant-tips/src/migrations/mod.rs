// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// Version 4.
///
/// For backward compatibility reasons, topsoil-tips uses `Treasury` for storage module prefix
/// before calling this migration. After calling this migration, it will get replaced with
/// own storage identifier.
pub mod v4;

/// A migration that unreserves all funds held in the context of this pallet.
pub mod unreserve_deposits;
