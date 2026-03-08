// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for the transaction storage proof processing.

use subsoil::runtime::traits::NumberFor;

subsoil::api::decl_runtime_apis! {
	/// Runtime API trait for transaction storage support.
	pub trait TransactionStorageApi {
		/// Get the actual value of a retention period in blocks.
		fn retention_period() -> NumberFor<Block>;
	}
}
