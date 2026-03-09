// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[derive(topsoil_core::DefaultNoBound)]
union Foo {
	field1: u32,
	field2: (),
}

fn main() {}
