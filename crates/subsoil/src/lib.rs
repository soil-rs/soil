#![cfg_attr(not(feature = "std"), no_std)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

pub extern crate alloc;

pub use subsoil_crypto_hashing as crypto_hashing;

#[allow(clippy::module_inception)]
pub mod std;
#[cfg(feature = "std")]
pub mod database;
#[cfg(feature = "std")]
pub mod panic_handler;
pub mod arithmetic;
pub mod wasm_interface;
pub mod metadata_ir;
pub mod tracing;
pub mod binary_merkle_tree;
pub mod externalities;
pub mod storage;
pub mod weights;
#[allow(clippy::module_inception)]
pub mod core;
pub mod keystore;
#[cfg(feature = "std")]
pub mod allocator;
pub mod runtime_interface;
pub mod trie;
pub mod state_machine;
pub mod io;
pub mod application_crypto;
pub mod runtime;
pub mod version;
pub mod api;
pub mod keyring;
pub mod crypto_ec_utils;
pub mod npos_elections;
pub mod inherents;
pub mod timestamp;
pub mod block_builder;
pub mod consensus;
pub mod mmr;

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
