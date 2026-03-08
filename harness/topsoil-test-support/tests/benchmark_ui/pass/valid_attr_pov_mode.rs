// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use topsoil_benchmarking::v2::*;
use topsoil_test_support::Config;

#[benchmarks]
mod benches {
	use super::*;

	#[benchmark(skip_meta, extra, pov_mode = Measured)]
	fn bench1() {
		#[block]
		{}
	}

	#[benchmark(pov_mode = Measured, extra, skip_meta)]
	fn bench2() {
		#[block]
		{}
	}

	#[benchmark(extra, pov_mode = Measured {
		Pallet: Measured,
		Pallet::Storage: MaxEncodedLen,
	}, skip_meta)]
	fn bench3() {
		#[block]
		{}
	}

	#[benchmark(skip_meta, extra, pov_mode = Measured {
		Pallet::Storage: MaxEncodedLen,
		Pallet::StorageSubKey: Measured,
	})]
	fn bench4() {
		#[block]
		{}
	}

	#[benchmark(pov_mode = MaxEncodedLen {
		Pallet::Storage: Measured,
		Pallet::StorageSubKey: Measured
	}, extra, skip_meta)]
	fn bench5() {
		#[block]
		{}
	}

	#[benchmark(pov_mode = MaxEncodedLen {
		Pallet::Storage: Measured,
		Pallet::Storage::Nested: Ignored
	}, extra, skip_meta)]
	fn bench6() {
		#[block]
		{}
	}
}

fn main() {}
