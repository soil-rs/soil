// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Simple ECDSA secp256k1 API.
//!
//! Provides an extension trait for [`subsoil::core::ecdsa::Public`] to do certain operations.

use subsoil::core::{crypto::ByteArray, ecdsa::Public};

/// Extension trait for [`Public`] to be used from inside the runtime.
///
/// # Note
///
/// This is needed because host functions cannot be called from within
/// `soil_core` due to cyclic dependencies  on `subsoil::io`.
pub trait ECDSAExt {
	/// Returns Ethereum address calculated from this ECDSA public key.
	fn to_eth_address(&self) -> Result<[u8; 20], ()>;
}

impl ECDSAExt for Public {
	fn to_eth_address(&self) -> Result<[u8; 20], ()> {
		use k256::{elliptic_curve::sec1::ToEncodedPoint, PublicKey};

		PublicKey::from_sec1_bytes(self.as_slice()).map_err(drop).and_then(|pub_key| {
			// uncompress the key
			let uncompressed = pub_key.to_encoded_point(false);
			// convert to ETH address
			<[u8; 20]>::try_from(
				subsoil::io::hashing::keccak_256(&uncompressed.as_bytes()[1..])[12..].as_ref(),
			)
			.map_err(drop)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use subsoil::core::{ecdsa, Pair};

	#[test]
	fn to_eth_address_works() {
		let pair = ecdsa::Pair::from_string("//Alice//password", None).unwrap();
		let eth_address = pair.public().to_eth_address().unwrap();
		assert_eq!(
			array_bytes::bytes2hex("0x", &eth_address),
			"0xdc1cce4263956850a3c8eb349dc6fc3f7792cb27"
		);
	}
}
