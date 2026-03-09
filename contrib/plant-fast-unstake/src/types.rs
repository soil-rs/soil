// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Types used in the Fast Unstake pallet.

use crate::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use subsoil::staking::{EraIndex, StakingInterface};
use topsoil_core::{
	traits::Currency, BoundedVec, CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound,
};

/// Maximum number of eras that we might check for a single staker.
///
/// In effect, it is the bonding duration, coming from [`Config::Staking`], plus one.
#[derive(scale_info::TypeInfo, codec::Encode, codec::Decode, codec::MaxEncodedLen)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
pub struct MaxChecking<T: Config>(core::marker::PhantomData<T>);
impl<T: Config> topsoil_core::traits::Get<u32> for MaxChecking<T> {
	fn get() -> u32 {
		T::Staking::bonding_duration() + 1
	}
}

#[docify::export]
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as topsoil_core::system::Config>::AccountId>>::Balance;
/// An unstake request.
///
/// This is stored in [`crate::Head`] storage item and points to the current unstake request that is
/// being processed.
#[derive(
	Encode, Decode, EqNoBound, PartialEqNoBound, CloneNoBound, TypeInfo, DebugNoBound, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct UnstakeRequest<T: Config> {
	/// This list of stashes are being processed in this request, and their corresponding deposit.
	pub stashes: BoundedVec<(T::AccountId, BalanceOf<T>), T::BatchSize>,
	/// The list of eras for which they have been checked.
	pub checked: BoundedVec<EraIndex, MaxChecking<T>>,
}
