// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use super::*;
use subsoil::impl_tx_ext_default;

/// This type serves as a marker extension to differentiate meta-transactions from regular
/// transactions. It implements the `TransactionExtension` trait and carries constant implicit data
/// ("_meta_tx").
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, DebugNoBound, DecodeWithMemTracking)]
#[scale_info(skip_type_params(T))]
pub struct MetaTxMarker<T> {
	_phantom: core::marker::PhantomData<T>,
}

impl<T> MetaTxMarker<T> {
	/// Creates new `TransactionExtension` with implicit meta tx marked.
	pub fn new() -> Self {
		Self { _phantom: Default::default() }
	}
}

impl<T: Config + Send + Sync> TransactionExtension<T::RuntimeCall> for MetaTxMarker<T> {
	const IDENTIFIER: &'static str = "MetaTxMarker";
	type Implicit = [u8; 8];
	type Val = ();
	type Pre = ();
	fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
		Ok(*b"_meta_tx")
	}
	fn weight(&self, _: &T::RuntimeCall) -> Weight {
		Weight::zero()
	}
	impl_tx_ext_default!(T::RuntimeCall; validate prepare);
}
