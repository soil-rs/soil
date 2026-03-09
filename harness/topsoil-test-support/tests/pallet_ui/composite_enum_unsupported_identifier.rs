// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_core::pallet]
mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::composite_enum]
	pub enum HoldReasons {}
}

fn main() {}