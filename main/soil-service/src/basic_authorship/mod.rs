// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Basic implementation of block-authoring logic.
//!
//! # Example
//!
//! ```
//! # use soil_service::basic_authorship::ProposerFactory;
//! # use soil_client::consensus::{Environment, Proposer, ProposeArgs};
//! # use subsoil::runtime::generic::BlockId;
//! # use std::{sync::Arc, time::Duration};
//! # use soil_test_node_runtime_client::{
//! #     runtime::Transfer, Sr25519Keyring,
//! #     DefaultTestClientBuilderExt, TestClientBuilderExt,
//! # };
//! # use soil_txpool::{BasicPool, FullChainApi};
//! # let client = Arc::new(soil_test_node_runtime_client::new());
//! # let spawner = subsoil::core::testing::TaskExecutor::new();
//! # let txpool = Arc::from(BasicPool::new_full(
//! #     Default::default(),
//! #     true.into(),
//! #     None,
//! #     spawner.clone(),
//! #     client.clone(),
//! # ));
//! // The first step is to create a `ProposerFactory`.
//! let mut proposer_factory = ProposerFactory::new(
//! 		spawner,
//! 		client.clone(),
//! 		txpool.clone(),
//! 		None,
//! 		None,
//! 	);
//!
//! // From this factory, we create a `Proposer`.
//! let proposer = proposer_factory.init(
//! 	&client.header(client.chain_info().genesis_hash).unwrap().unwrap(),
//! );
//!
//! // The proposer is created asynchronously.
//! let proposer = futures::executor::block_on(proposer).unwrap();
//!
//! // This `Proposer` allows us to create a block proposition.
//! // The proposer will grab transactions from the transaction pool, and put them into the block.
//! let future = Proposer::propose(
//! 	proposer,
//!     ProposeArgs {
//! 	    max_duration: Duration::from_secs(2),
//! 	    ..Default::default()
//!     }
//! );
//!
//! // We wait until the proposition is performed.
//! let block = futures::executor::block_on(future).unwrap();
//! println!("Generated block: {:?}", block.block);
//! ```

mod basic_authorship;

pub use self::basic_authorship::{Proposer, ProposerFactory, DEFAULT_BLOCK_SIZE_LIMIT};
pub use soil_client::consensus::ProposeArgs;
