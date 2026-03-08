// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! BLS12-381 crypto applications.
use super::{KeyTypeId, RuntimePublic};

use alloc::vec::Vec;

pub use crate::core::bls::{
	bls381::{BlsEngine as Bls381Engine, *},
	Pair as BLS_Pair, ProofOfPossession as BLSPoP,
};
use crate::core::{crypto::CryptoType, proof_of_possession::ProofOfPossessionVerifier};

mod app {
	crate::app_crypto!(super, crate::core::testing::BLS381);
}

#[cfg(feature = "full_crypto")]
pub use app::Pair as AppPair;
pub use app::{
	ProofOfPossession as AppProofOfPossession, Public as AppPublic, Signature as AppSignature,
};

impl RuntimePublic for Public {
	type Signature = Signature;
	type ProofOfPossession = ProofOfPossession;

	/// Dummy implementation. Returns an empty vector.
	fn all(_key_type: KeyTypeId) -> Vec<Self> {
		Vec::new()
	}

	fn generate_pair(key_type: KeyTypeId, seed: Option<Vec<u8>>) -> Self {
		crate::io::crypto::bls381_generate(key_type, seed)
	}

	/// Dummy implementation. Returns `None`.
	fn sign<M: AsRef<[u8]>>(&self, _key_type: KeyTypeId, _msg: &M) -> Option<Self::Signature> {
		None
	}

	/// Dummy implementation. Returns `false`.
	fn verify<M: AsRef<[u8]>>(&self, _msg: &M, _signature: &Self::Signature) -> bool {
		false
	}

	fn generate_proof_of_possession(
		&mut self,
		key_type: KeyTypeId,
		owner: &[u8],
	) -> Option<Self::ProofOfPossession> {
		crate::io::crypto::bls381_generate_proof_of_possession(key_type, self, owner)
	}

	fn verify_proof_of_possession(
		&self,
		owner: &[u8],
		proof_of_possession: &Self::ProofOfPossession,
	) -> bool {
		let pub_key = AppPublic::from(*self);
		<AppPublic as CryptoType>::Pair::verify_proof_of_possession(
			owner,
			&proof_of_possession,
			&pub_key,
		)
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		crate::core::crypto::ByteArray::to_raw_vec(self)
	}
}
