// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;
#[allow(unused_imports)]
use topsoil_test_support::Config;

#[benchmarks]
mod benchmarks {
	#[benchmark]
	fn bench() -> Result<u32, BenchmarkError> {
		#[block]
		{}
		Ok(10)
	}
}

fn main() {}
