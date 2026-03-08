// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Include sources generated from protobuf definitions.

pub(crate) mod bitswap {
	include!(concat!(env!("OUT_DIR"), "/bitswap.message.rs"));
}
