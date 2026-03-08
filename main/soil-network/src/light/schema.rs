// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Include sources generated from protobuf definitions.

pub(crate) mod v1 {
	pub(crate) mod light {
		include!(concat!(env!("OUT_DIR"), "/api.v1.light.rs"));
	}
}

#[cfg(test)]
mod tests {
	use prost::Message as _;

	#[test]
	fn empty_proof_encodes_correctly() {
		let encoded = super::v1::light::Response {
			response: Some(super::v1::light::response::Response::RemoteReadResponse(
				super::v1::light::RemoteReadResponse { proof: Some(Vec::new()) },
			)),
		}
		.encode_to_vec();

		// Make sure that the response contains one field of number 2 and wire type 2 (message),
		// then another field of number 2 and wire type 2 (bytes), then a length of 0.
		assert_eq!(encoded, vec![(2 << 3) | 2, 2, (2 << 3) | 2, 0]);
	}

	#[test]
	fn no_proof_encodes_correctly() {
		let encoded = super::v1::light::Response {
			response: Some(super::v1::light::response::Response::RemoteReadResponse(
				super::v1::light::RemoteReadResponse { proof: None },
			)),
		}
		.encode_to_vec();

		// Make sure that the response contains one field of number 2 and wire type 2 (message).
		assert_eq!(encoded, vec![(2 << 3) | 2, 0]);
	}

	#[test]
	fn proof_encodes_correctly() {
		let encoded = super::v1::light::Response {
			response: Some(super::v1::light::response::Response::RemoteReadResponse(
				super::v1::light::RemoteReadResponse { proof: Some(vec![1, 2, 3, 4]) },
			)),
		}
		.encode_to_vec();

		assert_eq!(encoded, vec![(2 << 3) | 2, 6, (2 << 3) | 2, 4, 1, 2, 3, 4]);
	}
}
