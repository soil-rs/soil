// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: MIT-0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use topsoil::prelude::storage::StorageDoubleMap;
use plant_assets::FrozenBalance;

// Implements [`FrozenBalance`] from [`topsoil-assets`], so it can understand how much of an
// account balance is frozen, and is able to signal to this pallet when to clear the state of an
// account.
impl<T: Config<I>, I: 'static> FrozenBalance<T::AssetId, T::AccountId, T::Balance>
	for Pallet<T, I>
{
	fn frozen_balance(asset: T::AssetId, who: &T::AccountId) -> Option<T::Balance> {
		FrozenBalances::<T, I>::get(asset, who)
	}

	fn died(asset: T::AssetId, who: &T::AccountId) {
		defensive_assert!(
			Freezes::<T, I>::get(asset.clone(), who).is_empty(),
			"The list of Freezes should be empty before allowing an account to die"
		);
		defensive_assert!(
			FrozenBalances::<T, I>::get(asset.clone(), who).is_none(),
			"There should not be a frozen balance before allowing to die"
		);

		FrozenBalances::<T, I>::remove(asset.clone(), who);
		Freezes::<T, I>::remove(asset, who);
	}

	fn contains_freezes(asset: T::AssetId) -> bool {
		Freezes::<T, I>::contains_prefix(asset)
	}
}

// Implement [`fungibles::Inspect`](topsoil_support::traits::fungibles::Inspect) as it is bound by
// [`fungibles::InspectFreeze`](topsoil_support::traits::fungibles::InspectFreeze) and
// [`fungibles::MutateFreeze`](topsoil_support::traits::fungibles::MutateFreeze). To do so, we'll
// re-export all of `topsoil-assets` implementation of the same trait.
impl<T: Config<I>, I: 'static> Inspect<T::AccountId> for Pallet<T, I> {
	type AssetId = T::AssetId;
	type Balance = T::Balance;

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		plant_assets::Pallet::<T, I>::total_issuance(asset)
	}

	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		plant_assets::Pallet::<T, I>::minimum_balance(asset)
	}

	fn total_balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		plant_assets::Pallet::<T, I>::total_balance(asset, who)
	}

	fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		plant_assets::Pallet::<T, I>::balance(asset, who)
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &T::AccountId,
		preservation: Preservation,
		force: Fortitude,
	) -> Self::Balance {
		plant_assets::Pallet::<T, I>::reducible_balance(asset, who, preservation, force)
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		provenance: Provenance,
	) -> DepositConsequence {
		plant_assets::Pallet::<T, I>::can_deposit(asset, who, amount, provenance)
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		plant_assets::Pallet::<T, I>::can_withdraw(asset, who, amount)
	}

	fn asset_exists(asset: Self::AssetId) -> bool {
		plant_assets::Pallet::<T, I>::asset_exists(asset)
	}
}

impl<T: Config<I>, I: 'static> InspectFreeze<T::AccountId> for Pallet<T, I> {
	type Id = T::RuntimeFreezeReason;

	fn balance_frozen(asset: Self::AssetId, id: &Self::Id, who: &T::AccountId) -> Self::Balance {
		let freezes = Freezes::<T, I>::get(asset, who);
		freezes.into_iter().find(|l| &l.id == id).map_or(Zero::zero(), |l| l.amount)
	}

	fn can_freeze(asset: Self::AssetId, id: &Self::Id, who: &T::AccountId) -> bool {
		let freezes = Freezes::<T, I>::get(asset, who);
		!freezes.is_full() || freezes.into_iter().any(|i| i.id == *id)
	}
}

impl<T: Config<I>, I: 'static> MutateFreeze<T::AccountId> for Pallet<T, I> {
	fn set_freeze(
		asset: Self::AssetId,
		id: &Self::Id,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if amount.is_zero() {
			return Self::thaw(asset, id, who);
		}
		let mut freezes = Freezes::<T, I>::get(asset.clone(), who);
		if let Some(i) = freezes.iter_mut().find(|i| &i.id == id) {
			i.amount = amount;
		} else {
			freezes
				.try_push(IdAmount { id: *id, amount })
				.map_err(|_| Error::<T, I>::TooManyFreezes)?;
		}
		Self::update_freezes(asset, who, freezes.as_bounded_slice())
	}

	fn extend_freeze(
		asset: Self::AssetId,
		id: &Self::Id,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if amount.is_zero() {
			return Ok(());
		}
		let mut freezes = Freezes::<T, I>::get(asset.clone(), who);
		if let Some(i) = freezes.iter_mut().find(|x| &x.id == id) {
			i.amount = i.amount.max(amount);
		} else {
			freezes
				.try_push(IdAmount { id: *id, amount })
				.map_err(|_| Error::<T, I>::TooManyFreezes)?;
		}
		Self::update_freezes(asset, who, freezes.as_bounded_slice())
	}

	fn thaw(asset: Self::AssetId, id: &Self::Id, who: &T::AccountId) -> DispatchResult {
		let mut freezes = Freezes::<T, I>::get(asset.clone(), who);
		freezes.retain(|f| &f.id != id);
		Self::update_freezes(asset, who, freezes.as_bounded_slice())
	}
}
