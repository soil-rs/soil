// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_support::derive_impl;

trait Shape {
	fn area(&self) -> u32;
}

struct SomeRectangle {}

#[topsoil_support::register_default_impl(SomeRectangle)]
impl Shape for SomeRectangle {
	fn area(&self) -> u32 {
		10
	}
}

struct SomeSquare {}

#[derive_impl(SomeRectangle)]
impl Shape for SomeSquare {}

#[test]
fn test_feature_parsing() {
	let square = SomeSquare {};
	assert_eq!(square.area(), 10);
}
