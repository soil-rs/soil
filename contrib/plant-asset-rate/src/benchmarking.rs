// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The crate's benchmarks.

use super::*;
use crate::{pallet as plant_asset_rate, Pallet as AssetRate};

use codec::Encode;
use subsoil::core::crypto::FromEntropy;
use topsoil_benchmarking::v2::*;
use topsoil_support::assert_ok;
use topsoil_system::RawOrigin;

/// Trait describing the factory function for the `AssetKind` parameter.
pub trait AssetKindFactory<AssetKind> {
	fn create_asset_kind(seed: u32) -> AssetKind;
}
impl<AssetKind> AssetKindFactory<AssetKind> for ()
where
	AssetKind: FromEntropy,
{
	fn create_asset_kind(seed: u32) -> AssetKind {
		AssetKind::from_entropy(&mut seed.encode().as_slice()).unwrap()
	}
}

const SEED: u32 = 1;

fn default_conversion_rate() -> FixedU128 {
	FixedU128::from_u32(1u32)
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create() -> Result<(), BenchmarkError> {
		let asset_kind: T::AssetKind = T::BenchmarkHelper::create_asset_kind(SEED);
		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(asset_kind.clone()), default_conversion_rate());

		assert_eq!(
			plant_asset_rate::ConversionRateToNative::<T>::get(asset_kind),
			Some(default_conversion_rate())
		);
		Ok(())
	}

	#[benchmark]
	fn update() -> Result<(), BenchmarkError> {
		let asset_kind: T::AssetKind = T::BenchmarkHelper::create_asset_kind(SEED);
		assert_ok!(AssetRate::<T>::create(
			RawOrigin::Root.into(),
			Box::new(asset_kind.clone()),
			default_conversion_rate()
		));

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(asset_kind.clone()), FixedU128::from_u32(2));

		assert_eq!(
			plant_asset_rate::ConversionRateToNative::<T>::get(asset_kind),
			Some(FixedU128::from_u32(2))
		);
		Ok(())
	}

	#[benchmark]
	fn remove() -> Result<(), BenchmarkError> {
		let asset_kind: T::AssetKind = T::BenchmarkHelper::create_asset_kind(SEED);
		assert_ok!(AssetRate::<T>::create(
			RawOrigin::Root.into(),
			Box::new(asset_kind.clone()),
			default_conversion_rate()
		));

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(asset_kind.clone()));

		assert!(plant_asset_rate::ConversionRateToNative::<T>::get(asset_kind).is_none());
		Ok(())
	}

	impl_benchmark_test_suite! { AssetRate, crate::mock::new_test_ext(), crate::mock::Test }
}
