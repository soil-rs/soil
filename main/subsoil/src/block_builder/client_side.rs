// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::BlockBuilder;

use crate::inherents::{InherentData, InherentDataProvider, InherentIdentifier};
use crate::runtime::traits::Block as BlockT;

/// Errors that occur when creating and checking on the client side.
#[derive(Debug)]
pub enum CheckInherentsError {
	/// Create inherents error.
	CreateInherentData(crate::inherents::Error),
	/// Client Error
	Client(crate::api::ApiError),
	/// Check inherents error
	CheckInherents(crate::inherents::Error),
	/// Unknown inherent error for identifier
	CheckInherentsUnknownError(InherentIdentifier),
}

/// Create inherent data and check that the inherents are valid.
pub async fn check_inherents<Block: BlockT, Client: crate::api::ProvideRuntimeApi<Block>>(
	client: std::sync::Arc<Client>,
	at_hash: Block::Hash,
	block: Block,
	inherent_data_providers: &impl InherentDataProvider,
) -> Result<(), CheckInherentsError>
where
	Client::Api: BlockBuilder<Block>,
{
	let inherent_data = inherent_data_providers
		.create_inherent_data()
		.await
		.map_err(CheckInherentsError::CreateInherentData)?;

	check_inherents_with_data(client, at_hash, block, inherent_data_providers, inherent_data).await
}

/// Check that the inherents are valid.
pub async fn check_inherents_with_data<
	Block: BlockT,
	Client: crate::api::ProvideRuntimeApi<Block>,
>(
	client: std::sync::Arc<Client>,
	at_hash: Block::Hash,
	block: Block,
	inherent_data_provider: &impl InherentDataProvider,
	inherent_data: InherentData,
) -> Result<(), CheckInherentsError>
where
	Client::Api: BlockBuilder<Block>,
{
	let res = client
		.runtime_api()
		.check_inherents(at_hash, block.into(), inherent_data)
		.map_err(CheckInherentsError::Client)?;

	if !res.ok() {
		for (id, error) in res.into_errors() {
			match inherent_data_provider.try_handle_error(&id, &error).await {
				Some(res) => res.map_err(CheckInherentsError::CheckInherents)?,
				None => return Err(CheckInherentsError::CheckInherentsUnknownError(id)),
			}
		}
	}

	Ok(())
}
