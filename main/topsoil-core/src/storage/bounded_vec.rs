// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Traits, types and structs to support putting a bounded vector into storage, as a raw value, map
//! or a double map.

use crate::{
	storage::{StorageDecodeLength, StorageTryAppend},
	traits::Get,
};
pub use subsoil::runtime::{BoundedSlice, BoundedVec};

impl<T, S> StorageDecodeLength for BoundedVec<T, S> {}

impl<T, S: Get<u32>> StorageTryAppend<T> for BoundedVec<T, S> {
	fn bound() -> usize {
		S::get() as usize
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::{traits::ConstU32, Twox128};
	use subsoil::io::TestExternalities;
	use subsoil::runtime::bounded_vec;

	#[crate::storage_alias]
	type Foo = StorageValue<Prefix, BoundedVec<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type FooMap = StorageMap<Prefix, Twox128, u32, BoundedVec<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type FooDoubleMap =
		StorageDoubleMap<Prefix, Twox128, u32, Twox128, u32, BoundedVec<u32, ConstU32<7>>>;

	#[test]
	fn decode_len_works() {
		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			Foo::put(bounded);
			assert_eq!(Foo::decode_len().unwrap(), 3);
		});

		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			FooMap::insert(1, bounded);
			assert_eq!(FooMap::decode_len(1).unwrap(), 3);
			assert!(FooMap::decode_len(0).is_none());
			assert!(FooMap::decode_len(2).is_none());
		});

		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			FooDoubleMap::insert(1, 1, bounded);
			assert_eq!(FooDoubleMap::decode_len(1, 1).unwrap(), 3);
			assert!(FooDoubleMap::decode_len(2, 1).is_none());
			assert!(FooDoubleMap::decode_len(1, 2).is_none());
			assert!(FooDoubleMap::decode_len(2, 2).is_none());
		});
	}
}
