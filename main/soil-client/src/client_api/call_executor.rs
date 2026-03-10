// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! A method call executor interface.

use crate::executor::{RuntimeVersion, RuntimeVersionOf};
use std::cell::RefCell;
use subsoil::core::traits::CallContext;
use subsoil::externalities::Extensions;
use subsoil::runtime::traits::{Block as BlockT, HashingFor};
use subsoil::state_machine::{OverlayedChanges, StorageProof};

use super::execution_extensions::ExecutionExtensions;
use subsoil::api::ProofRecorder;

/// Executor Provider
pub trait ExecutorProvider<Block: BlockT> {
	/// executor instance
	type Executor: CallExecutor<Block>;

	/// Get call executor reference.
	fn executor(&self) -> &Self::Executor;

	/// Get a reference to the execution extensions.
	fn execution_extensions(&self) -> &ExecutionExtensions<Block>;
}

/// Method call executor.
pub trait CallExecutor<B: BlockT>: RuntimeVersionOf {
	/// Externalities error type.
	type Error: subsoil::state_machine::Error;

	/// The backend used by the node.
	type Backend: super::backend::Backend<B>;

	/// Returns the [`ExecutionExtensions`].
	fn execution_extensions(&self) -> &ExecutionExtensions<B>;

	/// Execute a call to a contract on top of state in a block of given hash.
	///
	/// No changes are made.
	fn call(
		&self,
		at_hash: B::Hash,
		method: &str,
		call_data: &[u8],
		context: CallContext,
	) -> Result<Vec<u8>, crate::blockchain::Error>;

	/// Execute a contextual call on top of state in a block of a given hash.
	///
	/// No changes are made.
	/// Before executing the method, passed header is installed as the current header
	/// of the execution context.
	fn contextual_call(
		&self,
		at_hash: B::Hash,
		method: &str,
		call_data: &[u8],
		changes: &RefCell<OverlayedChanges<HashingFor<B>>>,
		proof_recorder: &Option<ProofRecorder<B>>,
		call_context: CallContext,
		extensions: &RefCell<Extensions>,
	) -> crate::blockchain::Result<Vec<u8>>;

	/// Extract RuntimeVersion of given block
	///
	/// No changes are made.
	fn runtime_version(
		&self,
		at_hash: B::Hash,
		call_context: CallContext,
	) -> Result<RuntimeVersion, crate::blockchain::Error>;

	/// Prove the execution of the given `method`.
	///
	/// No changes are made.
	fn prove_execution(
		&self,
		at_hash: B::Hash,
		method: &str,
		call_data: &[u8],
	) -> Result<(Vec<u8>, StorageProof), crate::blockchain::Error>;
}
