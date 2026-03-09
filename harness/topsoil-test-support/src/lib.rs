// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Minimal test support pallet.

// Make sure we fail compilation on warnings
#![warn(missing_docs)]
#![deny(warnings)]

pub use topsoil_core::dispatch::RawOrigin;
use topsoil_core::system::pallet_prelude::BlockNumberFor;

pub use self::pallet::*;

#[topsoil_core::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use crate::pallet_prelude::*;
	use topsoil_core::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The configuration trait.
	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A noop call.
		pub fn noop(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// A empty method.
		pub fn deposit_event(_event: impl Into<<T as topsoil_core::system::Config>::RuntimeEvent>) {}
	}

	/// The origin type.
	#[pallet::origin]
	pub type Origin<T> = RawOrigin<<T as topsoil_core::system::Config>::AccountId>;

	/// The error type.
	#[pallet::error]
	pub enum Error<T> {
		/// Test error documentation
		TestError,
		/// Error documentation
		/// with multiple lines
		AnotherError,
		/// Required by construct_runtime
		CallFiltered,
	}

	/// The event type.
	#[pallet::event]
	pub enum Event<T: Config> {
		/// The extrinsic is successful
		ExtrinsicSuccess,
		/// The extrinsic is failed
		ExtrinsicFailed,
		/// The ignored error
		Ignore(BlockNumberFor<T>),
	}
}

/// Ensure that the origin `o` represents the root. Returns `Ok` or an `Err` otherwise.
pub fn ensure_root<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<(), &'static str>
where
	OuterOrigin: Into<Result<RawOrigin<AccountId>, OuterOrigin>>,
{
	o.into().map(|_| ()).map_err(|_| "bad origin: expected to be a root origin")
}

/// Pallet prelude re-exports.
pub mod pallet_prelude {
	pub use crate::ensure_root;
	pub use topsoil_core::system::pallet_prelude::{BlockNumberFor, OriginFor};
}

/// Provides an implementation of [`topsoil_core::traits::Randomness`] that should only be used in
/// tests!
pub struct TestRandomness<T>(core::marker::PhantomData<T>);

impl<Output: codec::Decode + Default, T>
	topsoil_core::traits::Randomness<Output, BlockNumberFor<T>> for TestRandomness<T>
where
	T: topsoil_core::system::Config,
{
	fn random(subject: &[u8]) -> (Output, BlockNumberFor<T>) {
		use subsoil::runtime::traits::TrailingZeroInput;

		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			topsoil_core::system::Pallet::<T>::block_number(),
		)
	}
}
