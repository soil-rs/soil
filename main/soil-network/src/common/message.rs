// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Network packet message types. These get serialized and put into the lower level protocol
//! payload.

/// A unique ID of a request.
pub type RequestId = u64;
