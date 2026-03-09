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
pub use subsoil::runtime::WeakBoundedVec;

impl<T, S> StorageDecodeLength for WeakBoundedVec<T, S> {}

impl<T, S: Get<u32>> StorageTryAppend<T> for WeakBoundedVec<T, S> {
	fn bound() -> usize {
		S::get() as usize
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::Twox128;
	use subsoil::io::TestExternalities;
	use topsoil_core::traits::ConstU32;

	#[crate::storage_alias]
	type Foo = StorageValue<Prefix, WeakBoundedVec<u32, ConstU32<7>>>;
	#[crate::storage_alias]
	type FooMap = StorageMap<Prefix, Twox128, u32, WeakBoundedVec<u32, ConstU32<7>>>;
	#[crate::storage_alias]
	type FooDoubleMap =
		StorageDoubleMap<Prefix, Twox128, u32, Twox128, u32, WeakBoundedVec<u32, ConstU32<7>>>;

	#[test]
	fn decode_len_works() {
		TestExternalities::default().execute_with(|| {
			let bounded: WeakBoundedVec<u32, ConstU32<7>> = vec![1, 2, 3].try_into().unwrap();
			Foo::put(bounded);
			assert_eq!(Foo::decode_len().unwrap(), 3);
		});

		TestExternalities::default().execute_with(|| {
			let bounded: WeakBoundedVec<u32, ConstU32<7>> = vec![1, 2, 3].try_into().unwrap();
			FooMap::insert(1, bounded);
			assert_eq!(FooMap::decode_len(1).unwrap(), 3);
			assert!(FooMap::decode_len(0).is_none());
			assert!(FooMap::decode_len(2).is_none());
		});

		TestExternalities::default().execute_with(|| {
			let bounded: WeakBoundedVec<u32, ConstU32<7>> = vec![1, 2, 3].try_into().unwrap();
			FooDoubleMap::insert(1, 1, bounded);
			assert_eq!(FooDoubleMap::decode_len(1, 1).unwrap(), 3);
			assert!(FooDoubleMap::decode_len(2, 1).is_none());
			assert!(FooDoubleMap::decode_len(1, 2).is_none());
			assert!(FooDoubleMap::decode_len(2, 2).is_none());
		});
	}
}
