// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Types to put the runtime into safe mode.

/// Can put the runtime into a safe mode.
///
/// When the runtime entered safe mode, transaction processing for most general transactions is
/// paused.
pub trait SafeMode {
	/// Block number type.
	type BlockNumber;

	/// Whether safe mode is entered.
	fn is_entered() -> bool {
		Self::remaining().is_some()
	}

	/// How many more blocks safe mode will stay entered.
	///
	/// If this returns `0`, then safe mode will exit in the next block.
	fn remaining() -> Option<Self::BlockNumber>;

	/// Enter safe mode for `duration` blocks.
	///
	/// Should error when already entered with `AlreadyEntered`.
	fn enter(duration: Self::BlockNumber) -> Result<(), SafeModeError>;

	/// Extend safe mode for `duration` blocks.
	///
	/// Should error when not entered with `AlreadyExited`.
	fn extend(duration: Self::BlockNumber) -> Result<(), SafeModeError>;

	/// Exit safe mode immediately.
	///
	/// This takes effect already in the same block.
	fn exit() -> Result<(), SafeModeError>;
}

/// The error type for [`SafeMode`].
pub enum SafeModeError {
	/// Safe mode is already entered.
	AlreadyEntered,
	/// Safe mode is already exited.
	AlreadyExited,
	/// Unknown error.
	Unknown,
}

/// A trait to notify when the runtime enters or exits safe mode.
pub trait SafeModeNotify {
	/// Called when the runtime enters safe mode.
	fn entered();

	/// Called when the runtime exits safe mode.
	fn exited();
}

impl SafeModeNotify for () {
	fn entered() {}
	fn exited() {}
}
