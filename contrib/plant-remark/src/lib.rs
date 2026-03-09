// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Remark storage pallet. Indexes remarks and stores them off chain.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

extern crate alloc;

use alloc::vec::Vec;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
pub use weights::WeightInfo;

#[topsoil_core::pallet]
pub mod pallet {
	use super::*;
	use topsoil_core::pallet_prelude::*;
	use topsoil_core::system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {
		/// The overarching event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as topsoil_core::system::Config>::RuntimeEvent>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Attempting to store empty data.
		Empty,
		/// Attempted to call `store` outside of block execution.
		BadContext,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Index and store data off chain.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::store(remark.len() as u32))]
		pub fn store(origin: OriginFor<T>, remark: Vec<u8>) -> DispatchResultWithPostInfo {
			ensure!(!remark.is_empty(), Error::<T>::Empty);
			let sender = ensure_signed(origin)?;
			let content_hash = subsoil::io::hashing::blake2_256(&remark);
			let extrinsic_index = <topsoil_core::system::Pallet<T>>::extrinsic_index()
				.ok_or_else(|| Error::<T>::BadContext)?;
			subsoil::io::transaction_index::index(
				extrinsic_index,
				remark.len() as u32,
				content_hash,
			);
			Self::deposit_event(Event::Stored { sender, content_hash: content_hash.into() });
			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Stored data off chain.
		Stored { sender: T::AccountId, content_hash: subsoil::core::H256 },
	}
}
