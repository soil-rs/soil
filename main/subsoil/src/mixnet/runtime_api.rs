// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API for querying mixnet configuration and registering mixnodes.

use super::types::{Mixnode, MixnodesErr, SessionIndex, SessionStatus};
use alloc::vec::Vec;

crate::api::decl_runtime_apis! {
	/// API to query the mixnet session status and mixnode sets, and to register mixnodes.
	pub trait MixnetApi {
		/// Get the index and phase of the current session.
		fn session_status() -> SessionStatus;

		/// Get the mixnode set for the previous session.
		fn prev_mixnodes() -> Result<Vec<Mixnode>, MixnodesErr>;

		/// Get the mixnode set for the current session.
		fn current_mixnodes() -> Result<Vec<Mixnode>, MixnodesErr>;

		/// Try to register a mixnode for the next session.
		///
		/// If a registration extrinsic is submitted, `true` is returned. The caller should avoid
		/// calling `maybe_register` again for a few blocks, to give the submitted extrinsic a
		/// chance to get included.
		///
		/// With the above exception, `maybe_register` is designed to be called every block. Most
		/// of the time it will not do anything, for example:
		///
		/// - If it is not an appropriate time to submit a registration extrinsic.
		/// - If the local node has already registered a mixnode for the next session.
		/// - If the local node is not permitted to register a mixnode for the next session.
		///
		/// `session_index` should match `session_status().current_index`; if it does not, `false`
		/// is returned immediately.
		fn maybe_register(session_index: SessionIndex, mixnode: Mixnode) -> bool;
	}
}
