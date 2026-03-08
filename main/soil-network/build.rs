// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

const PROTOS: &[&str] =
	&["src/schema/api.v1.proto", "src/schema/bitswap.v1.2.0.proto", "src/schema/light.v1.proto"];

fn main() {
	prost_build::compile_protos(PROTOS, &["src/schema"]).unwrap();
}
