// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use alloc::collections::btree_map::BTreeMap;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use subsoil::arithmetic::traits::Zero;
use subsoil::core::{Get, TypedGet};
use subsoil::runtime::{DispatchError, DispatchResult};
use topsoil_core::{
	parameter_types,
	traits::{
		fungibles::{self, Dust},
		tokens::{
			self, DepositConsequence, Fortitude, Preservation, Provenance, WithdrawConsequence,
		},
	},
};

parameter_types! {
	static TestAssetOf: BTreeMap<(u32, Vec<u8>), Vec<u8>> = Default::default();
	static TestBalanceOf: BTreeMap<(u32, Vec<u8>, Vec<u8>), Vec<u8>> = Default::default();
	static TestHoldOf: BTreeMap<(u32, Vec<u8>, Vec<u8>, Vec<u8>), Vec<u8>> = Default::default();
}

pub struct TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>(
	core::marker::PhantomData<(Instance, AccountId, AssetId, MinimumBalance, HoldReason)>,
);
impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason,
	> fungibles::Inspect<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
	type AssetId = AssetId;
	type Balance = MinimumBalance::Type;

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		TestAssetOf::get()
			.get(&(Instance::get(), asset.encode()))
			.and_then(|data| Decode::decode(&mut &data[..]).ok())
			.unwrap_or_default()
	}

	fn active_issuance(asset: Self::AssetId) -> Self::Balance {
		Self::total_issuance(asset)
	}

	/// The minimum balance any single account may have.
	fn minimum_balance(_asset: Self::AssetId) -> Self::Balance {
		MinimumBalance::get()
	}

	fn total_balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		TestBalanceOf::get()
			.get(&(Instance::get(), asset.encode(), who.encode()))
			.and_then(|data| Decode::decode(&mut &data[..]).ok())
			.unwrap_or_default()
	}

	fn balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		Self::total_balance(asset, who)
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &AccountId,
		_preservation: Preservation,
		_force: Fortitude,
	) -> Self::Balance {
		Self::total_balance(asset, who)
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
		_provenance: Provenance,
	) -> DepositConsequence {
		if !Self::asset_exists(asset) {
			return DepositConsequence::UnknownAsset;
		}
		if amount + Self::balance(asset, who) < Self::minimum_balance(asset) {
			return DepositConsequence::BelowMinimum;
		}
		DepositConsequence::Success
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		if Self::reducible_balance(asset, who, Preservation::Expendable, Fortitude::Polite) < amount
		{
			return WithdrawConsequence::BalanceLow;
		}
		if Self::total_balance(asset, who) < Self::minimum_balance(asset) + amount {
			return WithdrawConsequence::WouldDie;
		}
		WithdrawConsequence::Success
	}

	fn asset_exists(asset: Self::AssetId) -> bool {
		TestAssetOf::get().contains_key(&(Instance::get(), asset.encode()))
	}
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason,
	> fungibles::Unbalanced<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
	fn handle_dust(_dust: Dust<AccountId, Self>) {}

	fn write_balance(
		asset: Self::AssetId,
		who: &AccountId,
		amount: Self::Balance,
	) -> Result<Option<Self::Balance>, DispatchError> {
		let mut tb = TestBalanceOf::get();
		let maybe_dust = if amount < MinimumBalance::get() {
			tb.remove(&(Instance::get(), asset.encode(), who.encode()));
			if amount.is_zero() {
				None
			} else {
				Some(amount)
			}
		} else {
			tb.insert((Instance::get(), asset.encode(), who.encode()), amount.encode());
			None
		};
		TestBalanceOf::set(tb);
		Ok(maybe_dust)
	}

	fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance) {
		let mut ta = TestAssetOf::get();
		ta.insert((Instance::get(), asset.encode()), amount.encode());
		TestAssetOf::set(ta);
	}
}

impl<
		Instance: Get<u32>,
		AccountId: Encode + Eq,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason,
	> fungibles::Mutate<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason,
	> fungibles::Balanced<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
	type OnDropCredit = fungibles::DecreaseIssuance<AccountId, Self>;
	type OnDropDebt = fungibles::IncreaseIssuance<AccountId, Self>;
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason: Encode + Decode + TypeInfo + 'static,
	> fungibles::InspectHold<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
	type Reason = HoldReason;

	fn total_balance_on_hold(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
		let asset = asset.encode();
		let who = who.encode();
		TestHoldOf::get()
			.iter()
			.filter(|(k, _)| k.0 == Instance::get() && k.1 == asset && k.2 == who)
			.filter_map(|(_, b)| Self::Balance::decode(&mut &b[..]).ok())
			.fold(Zero::zero(), |a, i| a + i)
	}

	fn balance_on_hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &AccountId,
	) -> Self::Balance {
		TestHoldOf::get()
			.get(&(Instance::get(), asset.encode(), who.encode(), reason.encode()))
			.and_then(|data| Decode::decode(&mut &data[..]).ok())
			.unwrap_or_default()
	}
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason: Encode + Decode + TypeInfo + 'static,
	> fungibles::UnbalancedHold<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
	fn set_balance_on_hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		let mut th = TestHoldOf::get();
		th.insert(
			(Instance::get(), asset.encode(), who.encode(), reason.encode()),
			amount.encode(),
		);
		TestHoldOf::set(th);
		Ok(())
	}
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason: Encode + Decode + TypeInfo + 'static,
	> fungibles::MutateHold<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason: Encode + Decode + TypeInfo + 'static,
		Balance: tokens::Balance,
	> fungibles::hold::DoneSlash<AssetId, HoldReason, AccountId, Balance>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
}

impl<
		Instance: Get<u32>,
		AccountId: Encode,
		AssetId: tokens::AssetId + Copy,
		MinimumBalance: TypedGet,
		HoldReason: Encode + Decode + TypeInfo + 'static,
	> fungibles::BalancedHold<AccountId>
	for TestFungibles<Instance, AccountId, AssetId, MinimumBalance, HoldReason>
where
	MinimumBalance::Type: tokens::Balance,
{
}
