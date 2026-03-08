// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

fn main() {
	#[cfg(feature = "std")]
	prost_build::compile_protos(
		&[
			"src/client/worker/schema/dht-v1.proto",
			"src/client/worker/schema/dht-v2.proto",
			"src/client/worker/schema/dht-v3.proto",
		],
		&["src/client/worker/schema"],
	)
	.unwrap();
}
