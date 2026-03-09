// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Traits for managing reward pools.

use crate::traits::schedule::DispatchTime;
use subsoil::runtime::{DispatchError, DispatchResult};

/// A trait for managing a rewards pool.
pub trait RewardsPool<AccountId> {
	type AssetId;
	type BlockNumber;
	type PoolId;
	type Balance;

	/// Create a new reward pool.
	///
	/// Parameters:
	/// - `creator`: The account to pay for on-chain stroage deposit;
	/// - `staked_asset_id`: the asset to be staked in the pool;
	/// - `reward_asset_id`: the asset to be distributed as rewards;
	/// - `reward_rate_per_block`: the amount of reward tokens distributed per block;
	/// - `expiry`: the block number at which the pool will cease to accumulate rewards. The
	///   [`DispatchTime::After`] variant evaluated at the execution time.
	/// - `admin`: the account allowed to extend the pool expiration, increase the rewards rate and
	///   receive the unutilized reward tokens back after the pool completion.
	fn create_pool(
		creator: &AccountId,
		staked_asset_id: Self::AssetId,
		reward_asset_id: Self::AssetId,
		reward_rate_per_block: Self::Balance,
		expiry: DispatchTime<Self::BlockNumber>,
		admin: &AccountId,
	) -> Result<Self::PoolId, DispatchError>;

	/// Modify a pool reward rate.
	///
	/// The reward rate can only be increased.
	///
	/// Only the pool admin may perform this operation.
	fn set_pool_reward_rate_per_block(
		admin: &AccountId,
		pool_id: Self::PoolId,
		new_reward_rate_per_block: Self::Balance,
	) -> DispatchResult;

	/// Modify a pool admin.
	///
	/// Only the pool admin may perform this operation.
	fn set_pool_admin(
		admin: &AccountId,
		pool_id: Self::PoolId,
		new_admin: AccountId,
	) -> DispatchResult;

	/// Set when the pool should expire.
	///
	/// The expiry block can only be extended.
	///
	/// Only the pool admin may perform this operation.
	fn set_pool_expiry_block(
		admin: &AccountId,
		pool_id: Self::PoolId,
		new_expiry: DispatchTime<Self::BlockNumber>,
	) -> DispatchResult;
}
