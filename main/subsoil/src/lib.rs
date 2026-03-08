// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

pub extern crate alloc;

pub use subsoil_crypto_hashing as crypto_hashing;

#[cfg(feature = "std")]
pub mod allocator;
pub mod api;
pub mod application_crypto;
pub mod arithmetic;
pub mod binary_merkle_tree;
pub mod block_builder;
pub mod consensus;
#[allow(clippy::module_inception)]
pub mod core;
pub mod crypto_ec_utils;
#[cfg(feature = "std")]
pub mod database;
pub mod externalities;
pub mod genesis_builder;
pub mod inherents;
pub mod io;
pub mod keyring;
pub mod keystore;
pub mod metadata_ir;
pub mod mixnet;
pub mod mmr;
pub mod npos_elections;
pub mod offchain_worker;
#[cfg(feature = "std")]
pub mod panic_handler;
pub mod runtime;
pub mod runtime_interface;
pub mod session;
pub mod staking;
pub mod state_machine;
#[allow(clippy::module_inception)]
pub mod std;
pub mod storage;
pub mod timestamp;
pub mod tracing;
pub mod trie;
pub mod txpool;
pub mod version;
pub mod wasm_interface;
pub mod weights;

/// Panic when the vectors are different, without taking the order into account.
#[macro_export]
macro_rules! assert_eq_uvec {
	( $x:expr, $y:expr $(,)? ) => {
		$crate::__assert_eq_uvec!($x, $y);
		$crate::__assert_eq_uvec!($y, $x);
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __assert_eq_uvec {
	( $x:expr, $y:expr ) => {
		$x.iter().for_each(|e| {
			if !$y.contains(e) {
				panic!("vectors not equal: {:?} != {:?}", $x, $y);
			}
		});
	};
}
