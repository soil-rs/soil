// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
