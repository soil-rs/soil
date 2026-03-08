// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Sr25519 crypto types.

use super::{KeyTypeId, RuntimePublic};

use alloc::vec::Vec;

use crate::core::proof_of_possession::NonAggregatable;
pub use crate::core::sr25519::*;

mod app {
	crate::app_crypto!(super, crate::core::testing::SR25519);
}

pub use app::{
	Pair as AppPair, ProofOfPossession as AppProofOfPossession, Public as AppPublic,
	Signature as AppSignature,
};

impl RuntimePublic for Public {
	type Signature = Signature;
	type ProofOfPossession = Signature;

	fn all(key_type: KeyTypeId) -> super::Vec<Self> {
		crate::io::crypto::sr25519_public_keys(key_type)
	}

	fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
		crate::io::crypto::sr25519_generate(key_type, seed)
	}

	fn sign<M: AsRef<[u8]>>(&self, key_type: KeyTypeId, msg: &M) -> Option<Self::Signature> {
		crate::io::crypto::sr25519_sign(key_type, self, msg.as_ref())
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		crate::io::crypto::sr25519_verify(signature, msg.as_ref(), self)
	}

	fn generate_proof_of_possession(
		&mut self,
		key_type: KeyTypeId,
		owner: &[u8],
	) -> Option<Self::ProofOfPossession> {
		let proof_of_possession_statement = Pair::proof_of_possession_statement(owner);
		crate::io::crypto::sr25519_sign(key_type, self, &proof_of_possession_statement)
	}

	fn verify_proof_of_possession(
		&self,
		owner: &[u8],
		proof_of_possession: &Self::ProofOfPossession,
	) -> bool {
		let proof_of_possession_statement = Pair::proof_of_possession_statement(owner);
		crate::io::crypto::sr25519_verify(
			&proof_of_possession,
			&proof_of_possession_statement,
			&self,
		)
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		crate::core::crypto::ByteArray::to_raw_vec(self)
	}
}
