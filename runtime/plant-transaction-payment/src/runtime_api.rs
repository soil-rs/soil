// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for transaction payment pallet.

use codec::Codec;
use subsoil::runtime::traits::MaybeDisplay;

pub use crate::{FeeDetails, InclusionFee, RuntimeDispatchInfo};

subsoil::api::decl_runtime_apis! {
	#[api_version(4)]
	pub trait TransactionPaymentApi<Balance> where
		Balance: Codec + MaybeDisplay,
	{
		fn query_info(uxt: Block::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance>;
		fn query_fee_details(uxt: Block::Extrinsic, len: u32) -> FeeDetails<Balance>;
		fn query_weight_to_fee(weight: subsoil::weights::Weight) -> Balance;
		fn query_length_to_fee(length: u32) -> Balance;
	}

	#[api_version(3)]
	pub trait TransactionPaymentCallApi<Balance, Call>
	where
		Balance: Codec + MaybeDisplay,
		Call: Codec,
	{
		/// Query information of a dispatch class, weight, and fee of a given encoded `Call`.
		fn query_call_info(call: Call, len: u32) -> RuntimeDispatchInfo<Balance>;

		/// Query fee details of a given encoded `Call`.
		fn query_call_fee_details(call: Call, len: u32) -> FeeDetails<Balance>;

		/// Query the output of the current `WeightToFee` given some input.
		fn query_weight_to_fee(weight: subsoil::weights::Weight) -> Balance;

		/// Query the output of the current `LengthToFee` given some input.
		fn query_length_to_fee(length: u32) -> Balance;
	}
}
