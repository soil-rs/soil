// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

trait Config {
	type C;
}

#[derive(topsoil_core::DefaultNoBound)]
enum Foo<T: Config> {
	Bar(T::C),
	Baz,
}

fn main() {}
