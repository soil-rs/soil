// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Generic Transaction Pool
//!
//! The pool is based on dependency graph between transactions
//! and their priority.
//! The pool is able to return an iterator that traverses transaction
//! graph in the correct order taking into account priorities and dependencies.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod future;
mod listener;
mod pool;
mod ready;
mod rotator;
pub(crate) mod tracked_map;
mod validated_pool;

pub mod base_pool;
pub mod watcher;

pub use self::pool::{
	BlockHash, ChainApi, ExtrinsicFor, ExtrinsicHash, NumberFor, Options, Pool, RawExtrinsicFor,
	TransactionFor, ValidateTransactionPriority, ValidatedTransactionFor,
};
pub use validated_pool::{
	BaseSubmitOutcome, EventDispatcher, IsValidator, ValidatedPoolSubmitOutcome,
	ValidatedTransaction,
};

pub(crate) use self::pool::CheckBannedBeforeVerify;
pub(crate) use listener::EventHandler;

#[cfg(doc)]
pub(crate) use validated_pool::ValidatedPool;
