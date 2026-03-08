// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Traits, types and structs to support a bounded `BTreeSet`.

pub use subsoil::runtime::BoundedBTreeSet;
use topsoil_support::storage::StorageDecodeNonDedupLength;

impl<T, S> StorageDecodeNonDedupLength for BoundedBTreeSet<T, S> {}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::Twox128;
	use alloc::collections::btree_set::BTreeSet;
	use subsoil::io::TestExternalities;
	use topsoil_support::traits::{ConstU32, Get};

	#[crate::storage_alias]
	type Foo = StorageValue<Prefix, BoundedBTreeSet<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type FooMap = StorageMap<Prefix, Twox128, u32, BoundedBTreeSet<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type FooDoubleMap =
		StorageDoubleMap<Prefix, Twox128, u32, Twox128, u32, BoundedBTreeSet<u32, ConstU32<7>>>;

	fn set_from_keys<T>(keys: &[T]) -> BTreeSet<T>
	where
		T: Ord + Copy,
	{
		keys.iter().copied().collect()
	}

	fn boundedset_from_keys<T, S>(keys: &[T]) -> BoundedBTreeSet<T, S>
	where
		T: Ord + Copy,
		S: Get<u32>,
	{
		set_from_keys(keys).try_into().unwrap()
	}

	#[test]
	fn decode_non_dedup_len_works() {
		TestExternalities::default().execute_with(|| {
			let bounded = boundedset_from_keys::<u32, ConstU32<7>>(&[1, 2, 3]);
			Foo::put(bounded);
			assert_eq!(Foo::decode_non_dedup_len().unwrap(), 3);
		});

		TestExternalities::default().execute_with(|| {
			let bounded = boundedset_from_keys::<u32, ConstU32<7>>(&[1, 2, 3]);
			FooMap::insert(1, bounded);
			assert_eq!(FooMap::decode_non_dedup_len(1).unwrap(), 3);
			assert!(FooMap::decode_non_dedup_len(0).is_none());
			assert!(FooMap::decode_non_dedup_len(2).is_none());
		});

		TestExternalities::default().execute_with(|| {
			let bounded = boundedset_from_keys::<u32, ConstU32<7>>(&[1, 2, 3]);
			FooDoubleMap::insert(1, 1, bounded);
			assert_eq!(FooDoubleMap::decode_non_dedup_len(1, 1).unwrap(), 3);
			assert!(FooDoubleMap::decode_non_dedup_len(2, 1).is_none());
			assert!(FooDoubleMap::decode_non_dedup_len(1, 2).is_none());
			assert!(FooDoubleMap::decode_non_dedup_len(2, 2).is_none());
		});
	}
}
