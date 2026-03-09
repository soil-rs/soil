// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Frame System benchmarks.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
pub mod extensions;

#[cfg(feature = "runtime-benchmarks")]
pub mod inner;

#[cfg(feature = "runtime-benchmarks")]
pub use inner::*;

#[cfg(all(feature = "runtime-benchmarks", test))]
pub(crate) mod mock;
