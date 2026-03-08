// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for the generic implementations of Extrinsic/Header/Block.

use super::DigestItem;
use codec::{Decode, Encode};

#[test]
fn system_digest_item_encoding() {
	let item = DigestItem::Consensus([1, 2, 3, 4], vec![5, 6, 7, 8]);
	let encoded = item.encode();
	assert_eq!(
		encoded,
		vec![
			4, // type = DigestItemType::Consensus
			1, 2, 3, 4, 16, 5, 6, 7, 8,
		]
	);

	let decoded: DigestItem = Decode::decode(&mut &encoded[..]).unwrap();
	assert_eq!(item, decoded);
}

#[test]
fn non_system_digest_item_encoding() {
	let item = DigestItem::Other(vec![10, 20, 30]);
	let encoded = item.encode();
	assert_eq!(
		encoded,
		vec![
			// type = DigestItemType::Other
			0,  // length of other data
			12, // authorities
			10, 20, 30,
		]
	);

	let decoded: DigestItem = Decode::decode(&mut &encoded[..]).unwrap();
	assert_eq!(item, decoded);
}
