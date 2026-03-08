// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;
use topsoil_support::parameter_types;
use topsoil_test_support::Config;

#[benchmarks]
mod benches {
	use super::*;

	#[allow(dead_code)]
	const MY_CONST: u32 = 100;

	#[allow(dead_code)]
	const fn my_fn() -> u32 {
		200
	}

	parameter_types! {
		const MyConst: u32 = MY_CONST;
	}

	#[benchmark(skip_meta, extra)]
	fn bench(a: Linear<{ MY_CONST * 2 }, { my_fn() + MyConst::get() }>) {
		let a = 2 + 2;
		#[block]
		{}
		assert_eq!(a, 4);
	}
}

fn main() {}
