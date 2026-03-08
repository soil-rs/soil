// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Tests for Nft fractionalization pallet.

use crate::{mock::*, *};

use fungible::{hold::Inspect as InspectHold, Mutate as MutateFungible};
use fungibles::{metadata::Inspect, InspectEnumerable};
use topsoil::{deps::subsoil::runtime::ModuleError, testing_prelude::*};
use TokenError::FundsUnavailable;

use plant_nfts::CollectionConfig;

fn assets() -> Vec<u32> {
	let mut s: Vec<_> = <<Test as Config>::Assets>::asset_ids().collect();
	s.sort();
	s
}

fn events() -> Vec<Event<Test>> {
	let result = System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let mock::RuntimeEvent::NftFractionalization(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect();

	System::reset_events();

	result
}

type AccountIdOf<Test> = <Test as topsoil_system::Config>::AccountId;

fn account(id: u8) -> AccountIdOf<Test> {
	[id; 32].into()
}

#[test]
fn fractionalize_should_work() {
	new_test_ext().execute_with(|| {
		let nft_collection_id = 0;
		let nft_id = 0;
		let asset_id = 0;
		let fractions = 1000;

		Balances::set_balance(&account(1), 100);
		Balances::set_balance(&account(2), 100);

		assert_ok!(Nfts::force_create(
			RuntimeOrigin::root(),
			account(1),
			CollectionConfig::default(),
		));
		assert_ok!(Nfts::mint(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			account(1),
			None,
		));

		assert_ok!(NftFractionalization::fractionalize(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			asset_id,
			account(2),
			fractions,
		));
		assert_eq!(assets(), vec![asset_id]);
		assert_eq!(Assets::balance(asset_id, account(2)), fractions);
		assert_eq!(Balances::total_balance_on_hold(&account(1)), 2);
		assert_eq!(
			String::from_utf8(
				<Assets as Inspect<<Test as topsoil_system::Config>::AccountId>>::name(0)
			)
			.unwrap(),
			"Frac 0-0"
		);
		assert_eq!(String::from_utf8(Assets::symbol(0)).unwrap(), "FRAC");
		assert_eq!(Nfts::owner(nft_collection_id, nft_id), Some(account(1)));
		assert_noop!(
			Nfts::transfer(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				account(2),
			),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [12, 0, 0, 0],
				message: Some("ItemLocked")
			})
		);

		let details = NftToAsset::<Test>::get((&nft_collection_id, &nft_id)).unwrap();
		assert_eq!(details.asset, asset_id);
		assert_eq!(details.fractions, fractions);

		assert!(events().contains(&Event::<Test>::NftFractionalized {
			nft_collection: nft_collection_id,
			nft: nft_id,
			fractions,
			asset: asset_id,
			beneficiary: account(2),
		}));

		// owner can't burn an already fractionalized NFT
		assert_noop!(
			Nfts::burn(RuntimeOrigin::signed(account(1)), nft_collection_id, nft_id),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [12, 0, 0, 0],
				message: Some("ItemLocked")
			})
		);

		// can't fractionalize twice
		assert_noop!(
			NftFractionalization::fractionalize(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				asset_id + 1,
				account(2),
				fractions,
			),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [12, 0, 0, 0],
				message: Some("ItemLocked")
			})
		);

		let nft_id = nft_id + 1;
		assert_noop!(
			NftFractionalization::fractionalize(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				asset_id,
				account(2),
				fractions,
			),
			Error::<Test>::NftNotFound
		);

		assert_ok!(Nfts::mint(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			account(2),
			None
		));
		assert_noop!(
			NftFractionalization::fractionalize(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				asset_id,
				account(2),
				fractions,
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn unify_should_work() {
	new_test_ext().execute_with(|| {
		let nft_collection_id = 0;
		let nft_id = 0;
		let asset_id = 0;
		let fractions = 1000;

		Balances::set_balance(&account(1), 100);
		Balances::set_balance(&account(2), 100);

		assert_ok!(Nfts::force_create(
			RuntimeOrigin::root(),
			account(1),
			CollectionConfig::default(),
		));
		assert_ok!(Nfts::mint(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			account(1),
			None,
		));
		assert_ok!(NftFractionalization::fractionalize(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			asset_id,
			account(2),
			fractions,
		));

		assert_noop!(
			NftFractionalization::unify(
				RuntimeOrigin::signed(account(2)),
				nft_collection_id + 1,
				nft_id,
				asset_id,
				account(1),
			),
			Error::<Test>::NftNotFractionalized
		);
		assert_noop!(
			NftFractionalization::unify(
				RuntimeOrigin::signed(account(2)),
				nft_collection_id,
				nft_id,
				asset_id + 1,
				account(1),
			),
			Error::<Test>::IncorrectAssetId
		);

		// can't unify the asset a user doesn't hold
		assert_noop!(
			NftFractionalization::unify(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				asset_id,
				account(1),
			),
			DispatchError::Token(FundsUnavailable)
		);

		assert_ok!(NftFractionalization::unify(
			RuntimeOrigin::signed(account(2)),
			nft_collection_id,
			nft_id,
			asset_id,
			account(1),
		));

		assert_eq!(Assets::balance(asset_id, account(2)), 0);
		assert_eq!(Balances::reserved_balance(&account(1)), 1);
		assert_eq!(Nfts::owner(nft_collection_id, nft_id), Some(account(1)));
		assert!(!NftToAsset::<Test>::contains_key((&nft_collection_id, &nft_id)));

		assert!(events().contains(&Event::<Test>::NftUnified {
			nft_collection: nft_collection_id,
			nft: nft_id,
			asset: asset_id,
			beneficiary: account(1),
		}));

		// validate we need to hold the full balance to un-fractionalize the NFT
		let asset_id = asset_id + 1;
		assert_ok!(NftFractionalization::fractionalize(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			asset_id,
			account(1),
			fractions,
		));
		assert_ok!(Assets::transfer(RuntimeOrigin::signed(account(1)), asset_id, account(2), 1));
		assert_eq!(Assets::balance(asset_id, account(1)), fractions - 1);
		assert_eq!(Assets::balance(asset_id, account(2)), 1);
		assert_noop!(
			NftFractionalization::unify(
				RuntimeOrigin::signed(account(1)),
				nft_collection_id,
				nft_id,
				asset_id,
				account(1),
			),
			DispatchError::Token(FundsUnavailable)
		);

		assert_ok!(Assets::transfer(RuntimeOrigin::signed(account(2)), asset_id, account(1), 1));
		assert_ok!(NftFractionalization::unify(
			RuntimeOrigin::signed(account(1)),
			nft_collection_id,
			nft_id,
			asset_id,
			account(2),
		));
		assert_eq!(Nfts::owner(nft_collection_id, nft_id), Some(account(2)));
	});
}
