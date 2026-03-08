// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for `decl_runtime_version` encoding compatibility.
//!
//! These tests verify that the duplicate `RuntimeVersion` struct inside
//! `subsoil-macros` (used by the proc-macro at compile time) produces an
//! encoding that is compatible with the canonical `subsoil::version::RuntimeVersion`.

use codec::Encode;
use std::borrow::Cow;

/// Mirror of the `RuntimeVersion` struct inside
/// `subsoil_macros::version::decl_runtime_version`. Must be kept in sync with
/// that definition so the encoding-compatibility test remains valid.
#[derive(Encode)]
struct RuntimeVersion {
	spec_name: String,
	impl_name: String,
	authoring_version: u32,
	spec_version: u32,
	impl_version: u32,
	apis: u8,
	transaction_version: u32,
	system_version: u8,
}

#[test]
fn version_can_be_deserialized() {
	let version_bytes = RuntimeVersion {
		spec_name: "hello".to_string(),
		impl_name: "world".to_string(),
		authoring_version: 10,
		spec_version: 265,
		impl_version: 1,
		apis: 0,
		transaction_version: 2,
		system_version: 1,
	}
	.encode();

	assert_eq!(
		subsoil::version::RuntimeVersion::decode_with_version_hint(
			&mut &version_bytes[..],
			Some(4)
		)
		.unwrap(),
		subsoil::version::RuntimeVersion {
			spec_name: "hello".into(),
			impl_name: "world".into(),
			authoring_version: 10,
			spec_version: 265,
			impl_version: 1,
			apis: Cow::Owned(vec![]),
			transaction_version: 2,
			system_version: 1,
		},
	);
}
