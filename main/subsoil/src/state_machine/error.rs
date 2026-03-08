// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

/// State Machine Errors
use core::fmt;

/// State Machine Error bound.
///
/// This should reflect Wasm error type bound for future compatibility.
pub trait Error: 'static + fmt::Debug + fmt::Display + Send + Sync {}

impl<T: 'static + fmt::Debug + fmt::Display + Send + Sync> Error for T {}

/// Externalities Error.
///
/// Externalities are not really allowed to have errors, since it's assumed that dependent code
/// would not be executed unless externalities were available. This is included for completeness,
/// and as a transition away from the pre-existing framework.
#[derive(Debug, Eq, PartialEq)]
#[allow(missing_docs)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum ExecutionError {
	#[cfg_attr(feature = "std", error("Backend error {0:?}"))]
	Backend(super::DefaultError),

	#[cfg_attr(feature = "std", error("`:code` entry does not exist in storage"))]
	CodeEntryDoesNotExist,

	#[cfg_attr(feature = "std", error("Unable to generate proof"))]
	UnableToGenerateProof,

	#[cfg_attr(feature = "std", error("Invalid execution proof"))]
	InvalidProof,
}
