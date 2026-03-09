// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! The crate's tests.

use super::*;
use crate::pallet as plant_asset_rate;
use mock::{new_test_ext, AssetRate, RuntimeOrigin, Test};
use subsoil::runtime::FixedU128;
use topsoil_core::{assert_noop, assert_ok};

const ASSET_ID: u32 = 42;

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		assert!(plant_asset_rate::ConversionRateToNative::<Test>::get(ASSET_ID).is_none());
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(0.1)
		));

		assert_eq!(
			plant_asset_rate::ConversionRateToNative::<Test>::get(ASSET_ID),
			Some(FixedU128::from_float(0.1))
		);
	});
}

#[test]
fn create_existing_throws() {
	new_test_ext().execute_with(|| {
		assert!(plant_asset_rate::ConversionRateToNative::<Test>::get(ASSET_ID).is_none());
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(0.1)
		));

		assert_noop!(
			AssetRate::create(
				RuntimeOrigin::root(),
				Box::new(ASSET_ID),
				FixedU128::from_float(0.1)
			),
			Error::<Test>::AlreadyExists
		);
	});
}

#[test]
fn remove_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(0.1)
		));

		assert_ok!(AssetRate::remove(RuntimeOrigin::root(), Box::new(ASSET_ID),));
		assert!(plant_asset_rate::ConversionRateToNative::<Test>::get(ASSET_ID).is_none());
	});
}

#[test]
fn remove_unknown_throws() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			AssetRate::remove(RuntimeOrigin::root(), Box::new(ASSET_ID),),
			Error::<Test>::UnknownAssetKind
		);
	});
}

#[test]
fn update_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(0.1)
		));
		assert_ok!(AssetRate::update(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(0.5)
		));

		assert_eq!(
			plant_asset_rate::ConversionRateToNative::<Test>::get(ASSET_ID),
			Some(FixedU128::from_float(0.5))
		);
	});
}

#[test]
fn update_unknown_throws() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			AssetRate::update(
				RuntimeOrigin::root(),
				Box::new(ASSET_ID),
				FixedU128::from_float(0.5)
			),
			Error::<Test>::UnknownAssetKind
		);
	});
}

#[test]
fn convert_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_float(2.51)
		));

		let conversion_from_asset = <AssetRate as ConversionFromAssetBalance<
			BalanceOf<Test>,
			<Test as plant_asset_rate::Config>::AssetKind,
			BalanceOf<Test>,
		>>::from_asset_balance(10, ASSET_ID);
		assert_eq!(conversion_from_asset.expect("Conversion rate exists for asset"), 25);

		let conversion_to_asset = <AssetRate as ConversionToAssetBalance<
			BalanceOf<Test>,
			<Test as plant_asset_rate::Config>::AssetKind,
			BalanceOf<Test>,
		>>::to_asset_balance(25, ASSET_ID);
		assert_eq!(conversion_to_asset.expect("Conversion rate exists for asset"), 9);
	});
}

#[test]
fn convert_unknown_throws() {
	new_test_ext().execute_with(|| {
		let conversion = <AssetRate as ConversionFromAssetBalance<
			BalanceOf<Test>,
			<Test as plant_asset_rate::Config>::AssetKind,
			BalanceOf<Test>,
		>>::from_asset_balance(10, ASSET_ID);
		assert!(conversion.is_err());
	});
}

#[test]
fn convert_overflow_throws() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRate::create(
			RuntimeOrigin::root(),
			Box::new(ASSET_ID),
			FixedU128::from_u32(0)
		));

		let conversion = <AssetRate as ConversionToAssetBalance<
			BalanceOf<Test>,
			<Test as plant_asset_rate::Config>::AssetKind,
			BalanceOf<Test>,
		>>::to_asset_balance(10, ASSET_ID);
		assert!(conversion.is_err());
	});
}
