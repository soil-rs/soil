// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test extracted from soil-rpc chain_head tests to break the soil-rpc → soil-service
//! dev-dependency cycle. This test requires `soil_service::client::new_with_backend`.

use assert_matches::assert_matches;
use jsonrpsee::{rpc_params, RpcModule};
use soil_client::block_builder::BlockBuilderBuilder;
use soil_client::blockchain::HeaderBackend;
use soil_client::consensus::BlockOrigin;
use soil_rpc::testing::TokioTestExecutor;
use soil_rpc::v2::chain_head::{
	ChainHead, ChainHeadApiServer, ChainHeadConfig, BestBlockChanged, FollowEvent, NewBlock,
};
use soil_service::client::new_with_backend;
use std::{sync::Arc, time::Duration};
use subsoil::runtime::traits::Block as BlockT;
use soil_test_node_runtime_client::{
	prelude::*,
	runtime::{Block, RuntimeApi},
	Client, ClientBlockImportExt, GenesisInit,
};

const MAX_PINNED_BLOCKS: usize = 32;
const MAX_PINNED_SECS: u64 = 60;
const MAX_OPERATIONS: usize = 16;
const MAX_LAGGING_DISTANCE: usize = 128;
const MAX_FOLLOW_SUBSCRIPTIONS_PER_CONNECTION: usize = 4;

async fn get_next_event<T: serde::de::DeserializeOwned>(
	sub: &mut jsonrpsee::core::server::Subscription,
) -> T {
	let (event, _sub_id) = tokio::time::timeout(std::time::Duration::from_secs(60), sub.next())
		.await
		.unwrap()
		.unwrap()
		.unwrap();
	event
}

async fn wait_pinned_references<Block: BlockT>(
	backend: &Arc<soil_client::client_api::in_mem::Backend<Block>>,
	hash: &Block::Hash,
	target: i64,
) {
	// Retry for at most 2 minutes.
	let mut retries = 120;
	while backend.pin_refs(hash).unwrap() != target {
		if retries == 0 {
			panic!("Expected target={} pinned references for hash={:?}", target, hash);
		}
		retries -= 1;

		tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
	}
}

#[tokio::test]
async fn pin_block_references() {
	// Manually construct an in-memory backend and client.
	let backend = Arc::new(soil_client::client_api::in_mem::Backend::new());
	let executor = soil_test_node_runtime_client::WasmExecutor::default();
	let client_config = soil_service::ClientConfig::default();

	let genesis_block_builder = soil_service::GenesisBlockBuilder::new(
		&soil_test_node_runtime_client::GenesisParameters::default().genesis_storage(),
		!client_config.no_genesis,
		backend.clone(),
		executor.clone(),
	)
	.unwrap();

	let client = Arc::new(
		new_with_backend::<_, _, Block, _, RuntimeApi>(
			backend.clone(),
			executor,
			genesis_block_builder,
			Box::new(TokioTestExecutor::default()),
			None,
			None,
			client_config,
		)
		.unwrap(),
	);

	let api = ChainHead::new(
		client.clone(),
		backend.clone(),
		Arc::new(TokioTestExecutor::default()),
		ChainHeadConfig {
			global_max_pinned_blocks: 3,
			subscription_max_pinned_duration: Duration::from_secs(MAX_PINNED_SECS),
			subscription_max_ongoing_operations: MAX_OPERATIONS,
			max_lagging_distance: MAX_LAGGING_DISTANCE,
			max_follow_subscriptions_per_connection: MAX_FOLLOW_SUBSCRIPTIONS_PER_CONNECTION,
			subscription_buffer_cap: MAX_PINNED_BLOCKS,
		},
	)
	.into_rpc();

	let mut sub = api.subscribe_unbounded("chainHead_v1_follow", [false]).await.unwrap();
	let sub_id = sub.subscription_id();
	let sub_id = serde_json::to_string(&sub_id).unwrap();

	let block = BlockBuilderBuilder::new(&*client)
		.on_parent_block(client.chain_info().genesis_hash)
		.with_parent_block_number(0)
		.build()
		.unwrap()
		.build()
		.unwrap()
		.block;
	let hash = block.header.hash();
	let block_hash = format!("{:?}", hash);
	client.import(BlockOrigin::Own, block.clone()).await.unwrap();

	// Ensure the imported block is propagated for this subscription.
	assert_matches!(
		get_next_event::<FollowEvent<String>>(&mut sub).await,
		FollowEvent::Initialized(_)
	);
	assert_matches!(
		get_next_event::<FollowEvent<String>>(&mut sub).await,
		FollowEvent::BestBlockChanged(_)
	);
	assert_matches!(
		get_next_event::<FollowEvent<String>>(&mut sub).await,
		FollowEvent::NewBlock(_)
	);
	assert_matches!(
		get_next_event::<FollowEvent<String>>(&mut sub).await,
		FollowEvent::BestBlockChanged(_)
	);

	// We need to wait a bit for:
	// 1. `NewBlock` and `BestBlockChanged` notifications to propagate to the chainHead
	// subscription. (pin_refs == 2)
	// 2. The chainHead to call `pin_blocks` only once for the `NewBlock`
	// notification (pin_refs == 3)
	// 3. Both notifications to go out of scope (pin_refs ==  1 (total 3 - dropped 2)).
	wait_pinned_references(&backend, &hash, 1).await;

	// To not exceed the number of pinned blocks, we need to unpin before the next import.
	let _res: () = api.call("chainHead_v1_unpin", rpc_params![&sub_id, &block_hash]).await.unwrap();

	// Make sure unpin clears out the reference.
	let refs = backend.pin_refs(&hash).unwrap();
	assert_eq!(refs, 0);

	// Add another 2 blocks and make sure we drop the subscription with the blocks pinned.
	let mut hashes = Vec::new();
	for _ in 0..2 {
		let block = BlockBuilderBuilder::new(&*client)
			.on_parent_block(client.chain_info().best_hash)
			.with_parent_block_number(client.chain_info().best_number)
			.build()
			.unwrap()
			.build()
			.unwrap()
			.block;
		let hash = block.hash();
		client.import(BlockOrigin::Own, block.clone()).await.unwrap();

		// Ensure the imported block is propagated for this subscription.
		assert_matches!(
			get_next_event::<FollowEvent<String>>(&mut sub).await,
			FollowEvent::NewBlock(_)
		);
		assert_matches!(
			get_next_event::<FollowEvent<String>>(&mut sub).await,
			FollowEvent::BestBlockChanged(_)
		);

		hashes.push(hash);
	}

	// Make sure the pin was propagated.
	for hash in &hashes {
		wait_pinned_references(&backend, hash, 1).await;
	}

	// Drop the subscription and expect the pinned blocks to be released.
	drop(sub);
	// The `chainHead` detects the subscription was terminated when it tries
	// to send another block.
	let block = BlockBuilderBuilder::new(&*client)
		.on_parent_block(client.chain_info().best_hash)
		.with_parent_block_number(client.chain_info().best_number)
		.build()
		.unwrap()
		.build()
		.unwrap()
		.block;
	client.import(BlockOrigin::Own, block.clone()).await.unwrap();

	for hash in &hashes {
		wait_pinned_references(&backend, &hash, 0).await;
	}
}
