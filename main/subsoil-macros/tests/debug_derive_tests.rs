// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![allow(deprecated)]

use subsoil_macros::RuntimeDebug;

#[derive(RuntimeDebug)]
struct Unnamed(u64, String);

#[derive(RuntimeDebug)]
struct Named {
	a: u64,
	b: String,
}

#[derive(RuntimeDebug)]
enum EnumLongName<A> {
	A,
	B(A, String),
	VariantLongName { a: A, b: String },
}

#[test]
fn should_display_proper_debug() {
	use self::EnumLongName as Enum;

	assert_eq!(format!("{:?}", Unnamed(1, "abc".into())), "Unnamed(1, \"abc\")");
	assert_eq!(format!("{:?}", Named { a: 1, b: "abc".into() }), "Named { a: 1, b: \"abc\" }");
	assert_eq!(format!("{:?}", Enum::<u64>::A), "EnumLongName::A");
	assert_eq!(format!("{:?}", Enum::B(1, "abc".into())), "EnumLongName::B(1, \"abc\")");
	assert_eq!(
		format!("{:?}", Enum::VariantLongName { a: 1, b: "abc".into() }),
		"EnumLongName::VariantLongName { a: 1, b: \"abc\" }"
	);
}
