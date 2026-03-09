// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utilities for dealing with crypto primitives. Sometimes we need to use these from inside WASM
//! contracts, where crypto calculations have weak performance.

pub mod ecdsa;
