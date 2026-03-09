// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Contains the [`Task`] trait, which defines a general-purpose way for defining and executing
//! service work, and supporting types.

use alloc::{vec, vec::IntoIter};
use codec::FullCodec;
use core::{fmt::Debug, iter::Iterator};
use scale_info::TypeInfo;
use subsoil::runtime::DispatchError;
use subsoil::weights::Weight;

/// Contain's re-exports of all the supporting types for the [`Task`] trait. Used in the macro
/// expansion of `RuntimeTask`.
#[doc(hidden)]
pub mod __private {
	pub use alloc::{vec, vec::IntoIter};
	pub use codec::FullCodec;
	pub use core::{fmt::Debug, iter::Iterator};
	pub use scale_info::TypeInfo;
	pub use subsoil::runtime::DispatchError;
	pub use subsoil::weights::Weight;
}

/// A general-purpose trait which defines a type of service work (i.e., work to performed by an
/// off-chain worker) including methods for enumerating, validating, indexing, and running
/// tasks of this type.
pub trait Task: Sized + FullCodec + TypeInfo + Clone + Debug + PartialEq + Eq {
	/// An [`Iterator`] over tasks of this type used as the return type for `enumerate`.
	type Enumeration: Iterator;

	/// Inspects the pallet's state and enumerates tasks of this type.
	fn iter() -> Self::Enumeration;

	/// Checks if a particular instance of this `Task` variant is a valid piece of work.
	///
	/// This is used to validate tasks for unsigned execution. Hence, it MUST be cheap
	/// with minimal to no storage reads. Else, it can make the blockchain vulnerable
	/// to DoS attacks.
	fn is_valid(&self) -> bool;

	/// Performs the work for this particular `Task` variant.
	fn run(&self) -> Result<(), DispatchError>;

	/// Returns the weight of executing this `Task`.
	fn weight(&self) -> Weight;

	/// A unique value representing this `Task` within the current pallet. Analogous to
	/// `call_index`, but for tasks.'
	///
	/// This value should be unique within the current pallet and can overlap with task indices
	/// in other pallets.
	fn task_index(&self) -> u32;
}

impl Task for () {
	type Enumeration = IntoIter<Self>;

	fn iter() -> Self::Enumeration {
		vec![].into_iter()
	}

	fn is_valid(&self) -> bool {
		true
	}

	fn run(&self) -> Result<(), DispatchError> {
		Ok(())
	}

	fn weight(&self) -> Weight {
		Weight::default()
	}

	fn task_index(&self) -> u32 {
		0
	}
}
