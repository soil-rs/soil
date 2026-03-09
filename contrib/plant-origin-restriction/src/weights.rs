// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use topsoil_core::weights::Weight;

/// Weight functions needed for pallet origins restriction.
pub trait WeightInfo {
	fn clean_usage() -> Weight;
	fn restrict_origin_tx_ext() -> Weight;
}

// For tests
impl WeightInfo for () {
	fn clean_usage() -> Weight { Weight::zero() }
	fn restrict_origin_tx_ext() -> Weight { Weight::zero() }
}
