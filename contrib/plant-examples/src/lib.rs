// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! # FRAME Pallet Examples
//!
//! This crate contains a collection of simple examples of FRAME pallets, demonstrating useful
//! features in action. It is not intended to be used in production.
//!
//! ## Pallets
//!
//! - [`plant_example_basic`]: This pallet demonstrates concepts, APIs and structures common to
//!   most FRAME runtimes.
//!
//! - [`plant_example_offchain_worker`]: This pallet demonstrates concepts, APIs and structures
//!   common to most offchain workers.
//!
//! - [`plant_default_config_example`]: This pallet demonstrates different ways to implement the
//!   `Config` trait of pallets.
//!
//! - [`plant_dev_mode`]: This pallet demonstrates the ease of requirements for a pallet in "dev
//!   mode".
//!
//! - [`plant_example_kitchensink`]: This pallet demonstrates a catalog of all FRAME macros in use
//!   and their various syntax options.
//!
//! - [`plant_example_split`]: A simple example of a FRAME pallet demonstrating the ability to
//!   split sections across multiple files.
//!
//! - [`plant_example_frame_crate`]: Example pallet showcasing how one can be built using only the
//! `frame` umbrella crate.
//!
//! - [`plant_example_single_block_migrations`]: An example pallet demonstrating best-practices for
//!   writing storage migrations.
//!
//! - [`plant_example_authorization_tx_extension`]: An example `TransactionExtension` that
//!   authorizes a custom origin through signature validation, along with two support pallets to
//!   showcase the usage.
//!
//! **Tip**: Use `cargo doc --package <pallet-name> --open` to view each pallet's documentation.
