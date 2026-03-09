// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! # Dynamic Allocation Pool (DAP) Pallet
//!
//! This pallet implements `OnUnbalanced` to collect funds (e.g., slashes) into a buffer account
//! instead of burning them. The buffer account must be pre-funded with at least ED (existential
//! deposit), e.g., via balances genesis config or a transfer. If the buffer account is not
//! pre-funded, deposits below ED will be silently burned.
//!
//! Incoming funds are deactivated to exclude them from governance voting.
//! When DAP distributes funds (e.g., to validators, nominators, treasury, collators), those funds
//! must be reactivated before transfer.
//!
//! Future phases will add:
//! - `FundingSource` (request_funds) for pulling funds
//! - Issuance curve and minting logic
//! - Distribution rules and scheduling

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub(crate) mod mock;
#[cfg(test)]
mod tests;

extern crate alloc;

use topsoil_core::{
	defensive,
	pallet_prelude::*,
	traits::{
		fungible::{Balanced, Credit, Inspect, Mutate, Unbalanced},
		Imbalance, OnUnbalanced,
	},
	PalletId,
};

pub use pallet::*;

const LOG_TARGET: &str = "runtime::dap";

/// Type alias for balance.
pub type BalanceOf<T> =
	<<T as Config>::Currency as Inspect<<T as topsoil_core::system::Config>::AccountId>>::Balance;

#[topsoil_core::pallet]
pub mod pallet {
	use super::*;
	use topsoil_core::{subsoil::runtime::traits::AccountIdConversion, traits::StorageVersion};

	/// The in-code storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		/// The currency type (new fungible traits).
		type Currency: Inspect<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Balanced<Self::AccountId>;

		/// The pallet ID used to derive the buffer account.
		///
		/// Each runtime should configure a unique ID to avoid collisions if multiple
		/// DAP instances are used.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	impl<T: Config> Pallet<T> {
		/// Get the DAP buffer account
		/// NOTE: We may need more accounts in the future, for instance, to manage the strategic
		/// reserve. We will add them as necessary, generating them with additional seed.
		pub fn buffer_account() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}

/// Type alias for credit (negative imbalance - funds that were slashed/removed).
/// This is for the `fungible::Balanced` trait as used by staking-async.
pub type CreditOf<T> = Credit<<T as topsoil_core::system::Config>::AccountId, <T as Config>::Currency>;

/// Implementation of OnUnbalanced for the fungible::Balanced trait.
/// Example: use as `type Slash = Dap` in staking-async config.
impl<T: Config> OnUnbalanced<CreditOf<T>> for Pallet<T> {
	fn on_nonzero_unbalanced(amount: CreditOf<T>) {
		let buffer = Self::buffer_account();
		let numeric_amount = amount.peek();

		// Resolve should never fail because:
		// - can_deposit on destination succeeds since buffer exists (created with provider at
		//   genesis/runtime upgrade so no ED issue)
		// - amount is guaranteed non-zero by the trait method signature
		// The only failure would be overflow on destination.
		let _ = T::Currency::resolve(&buffer, amount)
			.inspect_err(|_| {
				defensive!("🚨 Failed to deposit slash to DAP buffer - funds burned, it should never happen!");
			})
			.inspect(|_| {
				// Mark funds as inactive so they don't participate in governance voting.
				// Only deactivate on success; if resolve failed, tokens were burned.
				<T::Currency as Unbalanced<T::AccountId>>::deactivate(numeric_amount);
				log::debug!(
					target: LOG_TARGET,
					"💸 Deposited slash of {numeric_amount:?} to DAP buffer"
				);
			});
	}
}
