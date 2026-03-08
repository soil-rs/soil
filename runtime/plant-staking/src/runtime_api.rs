// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for the staking pallet.

use codec::Codec;

subsoil::api::decl_runtime_apis! {
	pub trait StakingApi<Balance, AccountId>
		where
			Balance: Codec,
			AccountId: Codec,
	{
		fn nominations_quota(balance: Balance) -> u32;
		fn eras_stakers_page_count(era: subsoil::staking::EraIndex, account: AccountId) -> subsoil::staking::Page;
		fn pending_rewards(era: subsoil::staking::EraIndex, account: AccountId) -> bool;
	}
}
