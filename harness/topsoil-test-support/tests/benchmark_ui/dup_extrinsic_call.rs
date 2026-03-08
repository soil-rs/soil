// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;
#[allow(unused_imports)]
use topsoil_test_support::Config;

#[benchmarks]
mod benches {
	use super::*;

	#[benchmark]
	fn bench() {
		let a = 2 + 2;
		#[extrinsic_call]
		_(stuff);
		#[extrinsic_call]
		_(other_stuff);
		assert_eq!(a, 4);
	}
}

fn main() {}
