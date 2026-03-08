// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests extracted from subsoil-crypto-hashing to break the
//! subsoil-crypto-hashing → subsoil-macros dev-dependency cycle.
//!
//! These tests verify that the runtime hash functions produce the same output
//! as the compile-time `subsoil_macros::*` macros.

use subsoil_crypto_hashing::*;

#[test]
fn blake2b() {
	assert_eq!(subsoil_macros::blake2b_64!(b""), blake2_64(b"")[..]);
	assert_eq!(subsoil_macros::blake2b_256!(b"test"), blake2_256(b"test")[..]);
	assert_eq!(subsoil_macros::blake2b_512!(b""), blake2_512(b"")[..]);
}

#[test]
fn keccak() {
	assert_eq!(subsoil_macros::keccak_256!(b"test"), keccak_256(b"test")[..]);
	assert_eq!(subsoil_macros::keccak_512!(b"test"), keccak_512(b"test")[..]);
}

#[test]
fn sha2() {
	assert_eq!(subsoil_macros::sha2_256!(b"test"), sha2_256(b"test")[..]);
}

#[test]
fn twox() {
	assert_eq!(subsoil_macros::twox_128!(b"test"), twox_128(b"test")[..]);
	assert_eq!(subsoil_macros::twox_64!(b""), twox_64(b"")[..]);
}

#[test]
fn twox_concats() {
	assert_eq!(
		subsoil_macros::twox_128!(b"test", b"123", b"45", b"", b"67890"),
		twox_128(&b"test1234567890"[..]),
	);
	assert_eq!(
		subsoil_macros::twox_128!(b"test", test, b"45", b"", b"67890"),
		twox_128(&b"testtest4567890"[..]),
	);
}
