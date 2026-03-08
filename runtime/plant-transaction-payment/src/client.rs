// This file is part of Substrate.
//
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

//! RPC interface for the transaction payment pallet.

use std::sync::Arc;

use codec::{Codec, Decode};
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::{
		error::{ErrorCode, ErrorObject},
		ErrorObjectOwned,
	},
};
use soil_client::blockchain::HeaderBackend;
use soil_rpc::number::NumberOrHex;
use subsoil::api::ProvideRuntimeApi;
use subsoil::core::Bytes;
use subsoil::runtime::traits::{Block as BlockT, MaybeDisplay};

pub use crate::TransactionPaymentApi as TransactionPaymentRuntimeApi;
use crate::{FeeDetails, InclusionFee, RuntimeDispatchInfo};

#[rpc(client, server)]
pub trait TransactionPaymentApi<BlockHash, ResponseType> {
	#[method(name = "payment_queryInfo")]
	fn query_info(&self, encoded_xt: Bytes, at: Option<BlockHash>) -> RpcResult<ResponseType>;

	#[method(name = "payment_queryFeeDetails")]
	fn query_fee_details(
		&self,
		encoded_xt: Bytes,
		at: Option<BlockHash>,
	) -> RpcResult<FeeDetails<NumberOrHex>>;
}

/// Provides RPC methods to query a dispatchable's class, weight and fee.
pub struct TransactionPayment<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> TransactionPayment<C, P> {
	/// Creates a new instance of the TransactionPayment Rpc helper.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, Balance>
	TransactionPaymentApiServer<
		<Block as BlockT>::Hash,
		RuntimeDispatchInfo<Balance, subsoil::weights::Weight>,
	> for TransactionPayment<C, Block>
where
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: TransactionPaymentRuntimeApi<Block, Balance>,
	Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex> + Send + Sync + 'static,
{
	fn query_info(
		&self,
		encoded_xt: Bytes,
		at: Option<Block::Hash>,
	) -> RpcResult<RuntimeDispatchInfo<Balance, subsoil::weights::Weight>> {
		let api = self.client.runtime_api();
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

		let encoded_len = encoded_xt.len() as u32;

		let uxt: Block::Extrinsic = Decode::decode(&mut &*encoded_xt).map_err(|e| {
			ErrorObject::owned(
				Error::DecodeError.into(),
				"Unable to query dispatch info.",
				Some(format!("{:?}", e)),
			)
		})?;

		fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
			ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
		}

		let res = api
			.query_info(at_hash, uxt, encoded_len)
			.map_err(|e| map_err(e, "Unable to query dispatch info."))?;

		Ok(RuntimeDispatchInfo {
			weight: res.weight,
			class: res.class,
			partial_fee: res.partial_fee,
		})
	}

	fn query_fee_details(
		&self,
		encoded_xt: Bytes,
		at: Option<Block::Hash>,
	) -> RpcResult<FeeDetails<NumberOrHex>> {
		let api = self.client.runtime_api();
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

		let encoded_len = encoded_xt.len() as u32;

		let uxt: Block::Extrinsic = Decode::decode(&mut &*encoded_xt).map_err(|e| {
			ErrorObject::owned(
				Error::DecodeError.into(),
				"Unable to query fee details.",
				Some(format!("{:?}", e)),
			)
		})?;
		let fee_details = api.query_fee_details(at_hash, uxt, encoded_len).map_err(|e| {
			ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query fee details.",
				Some(e.to_string()),
			)
		})?;

		let try_into_rpc_balance = |value: Balance| {
			value.try_into().map_err(|_| {
				ErrorObject::owned(
					ErrorCode::InvalidParams.code(),
					format!("{} doesn't fit in NumberOrHex representation", value),
					None::<()>,
				)
			})
		};

		Ok(FeeDetails {
			inclusion_fee: if let Some(inclusion_fee) = fee_details.inclusion_fee {
				Some(InclusionFee {
					base_fee: try_into_rpc_balance(inclusion_fee.base_fee)?,
					len_fee: try_into_rpc_balance(inclusion_fee.len_fee)?,
					adjusted_weight_fee: try_into_rpc_balance(inclusion_fee.adjusted_weight_fee)?,
				})
			} else {
				None
			},
			tip: Default::default(),
		})
	}
}
