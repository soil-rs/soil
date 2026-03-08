// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Substrate Blake2b Hasher implementation

pub mod blake2 {
	use crate::core::hash::H256;
	use hash256_std_hasher::Hash256StdHasher;
	use hash_db::Hasher;

	/// Concrete implementation of Hasher using Blake2b 256-bit hashes
	#[derive(Debug)]
	pub struct Blake2Hasher;

	impl Hasher for Blake2Hasher {
		type Out = H256;
		type StdHasher = Hash256StdHasher;
		const LENGTH: usize = 32;

		fn hash(x: &[u8]) -> Self::Out {
			crate::crypto_hashing::blake2_256(x).into()
		}
	}
}

pub mod keccak {
	use crate::core::hash::H256;
	use hash256_std_hasher::Hash256StdHasher;
	use hash_db::Hasher;

	/// Concrete implementation of Hasher using Keccak 256-bit hashes
	#[derive(Debug)]
	pub struct KeccakHasher;

	impl Hasher for KeccakHasher {
		type Out = H256;
		type StdHasher = Hash256StdHasher;
		const LENGTH: usize = 32;

		fn hash(x: &[u8]) -> Self::Out {
			crate::crypto_hashing::keccak_256(x).into()
		}
	}
}
