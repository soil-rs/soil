// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[topsoil_support::pallet]
#[allow(unused_imports)]
mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::error]
	pub enum Error<T> {
		CustomError(crate::MyError),
	}
}

#[derive(scale_info::TypeInfo, codec::Encode, codec::Decode, codec::DecodeWithMemTracking)]
enum MyError {}

fn main() {}
