// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Supporting types for try-runtime, testing and dry-running commands.

pub use topsoil_support::traits::{TryStateSelect, UpgradeCheckSelect};
use topsoil_support::weights::Weight;

subsoil::api::decl_runtime_apis! {
	/// Runtime api for testing the execution of a runtime upgrade.
	pub trait TryRuntime {
		/// dry-run runtime upgrades, returning the total weight consumed.
		///
		/// This should do EXACTLY the same operations as the runtime would have done in the case of
		/// a runtime upgrade (e.g. pallet ordering must be the same)
		///
		/// Returns the consumed weight of the migration in case of a successful one, combined with
		/// the total allowed block weight of the runtime.
		///
		/// If `checks` is `true`, `pre_migrate` and `post_migrate` of each migration and
		/// `try_state` of all pallets will be executed. Else, no. If checks are executed, the PoV
		/// tracking is likely inaccurate.
		fn on_runtime_upgrade(checks: UpgradeCheckSelect) -> (Weight, Weight);

		/// Execute the given block, but optionally disable state-root and signature checks.
		///
		/// Optionally, a number of `try_state` hooks can also be executed after the block
		/// execution.
		fn execute_block(
			block: Block::LazyBlock,
			state_root_check: bool,
			signature_check: bool,
			try_state: TryStateSelect,
		) -> Weight;
	}
}
