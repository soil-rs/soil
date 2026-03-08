// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Parameters pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
#[cfg(test)]
use crate::Pallet as Parameters;

use topsoil_benchmarking::v2::*;

#[benchmarks(where T::RuntimeParameters: Default)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_parameter() -> Result<(), BenchmarkError> {
		let kv = T::RuntimeParameters::default();
		let k = kv.clone().into_parts().0;

		let origin =
			T::AdminOrigin::try_successful_origin(&k).map_err(|_| BenchmarkError::Weightless)?;

		#[extrinsic_call]
		_(origin as T::RuntimeOrigin, kv);

		Ok(())
	}

	impl_benchmark_test_suite! {
		Parameters,
		crate::tests::mock::new_test_ext(),
		crate::tests::mock::Runtime,
	}
}
