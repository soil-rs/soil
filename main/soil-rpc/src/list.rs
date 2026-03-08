// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! RPC a lenient list or value type.

use serde::{Deserialize, Serialize};

/// RPC list or value wrapper.
///
/// For some RPCs it's convenient to call them with either
/// a single value or a whole list of values to get a proper response.
/// In theory you could do a batch query, but it's:
/// 1. Less convenient in client libraries
/// 2. If the response value is small, the protocol overhead might be dominant.
///
/// Also it's nice to be able to maintain backward compatibility for methods that
/// were initially taking a value and now we want to expand them to take a list.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ListOrValue<T> {
	/// A list of values of given type.
	List(Vec<T>),
	/// A single value of given type.
	Value(T),
}

impl<T> ListOrValue<T> {
	/// Map every contained value using function `F`.
	///
	/// This allows to easily convert all values in any of the variants.
	pub fn map<F: Fn(T) -> X, X>(self, f: F) -> ListOrValue<X> {
		match self {
			ListOrValue::List(v) => ListOrValue::List(v.into_iter().map(f).collect()),
			ListOrValue::Value(v) => ListOrValue::Value(f(v)),
		}
	}
}

impl<T> From<T> for ListOrValue<T> {
	fn from(n: T) -> Self {
		ListOrValue::Value(n)
	}
}

impl<T> From<Vec<T>> for ListOrValue<T> {
	fn from(n: Vec<T>) -> Self {
		ListOrValue::List(n)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_deser;

	#[test]
	fn should_serialize_and_deserialize() {
		assert_deser(r#"5"#, ListOrValue::Value(5_u64));
		assert_deser(r#""str""#, ListOrValue::Value("str".to_string()));
		assert_deser(r#"[1,2,3]"#, ListOrValue::List(vec![1_u64, 2_u64, 3_u64]));
	}
}
