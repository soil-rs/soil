// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use codec::{Decode, DecodeWithMemTracking, Encode};
use topsoil_core::PalletError;

#[topsoil_core::pallet]
#[allow(unused_imports)]
pub mod pallet {
	#[pallet::config]
	pub trait Config: topsoil_core::system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::error]
	pub enum Error<T> {
		CustomError(crate::MyError),
	}
}

#[derive(Encode, Decode, DecodeWithMemTracking, PalletError, scale_info::TypeInfo)]
pub enum MyError {
	Foo,
	Bar,
	Baz(NestedError),
	Struct(MyStruct),
	Wrapper(Wrapper),
}

#[derive(Encode, Decode, DecodeWithMemTracking, PalletError, scale_info::TypeInfo)]
pub enum NestedError {
	Quux,
}

#[derive(Encode, Decode, DecodeWithMemTracking, PalletError, scale_info::TypeInfo)]
pub struct MyStruct {
	field: u8,
}

#[derive(Encode, Decode, DecodeWithMemTracking, PalletError, scale_info::TypeInfo)]
pub struct Wrapper(bool);

fn main() {}
