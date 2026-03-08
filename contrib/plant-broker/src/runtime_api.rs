// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Runtime API definition for the FRAME Broker pallet.

use codec::Codec;
use subsoil::runtime::DispatchError;

subsoil::api::decl_runtime_apis! {
	pub trait BrokerApi<Balance>
	where
		Balance: Codec
	{
		/// If there is an ongoing sale returns the current price of a core.
		fn sale_price() -> Result<Balance, DispatchError>;
	}
}
