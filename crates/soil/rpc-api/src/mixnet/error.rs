// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Mixnet RPC module errors.

use jsonrpsee::types::error::{ErrorObject, ErrorObjectOwned};
use soil_network::mixnet::{Error as MixnetError, PostErr, RemoteErr, TopologyErr};

/// Mixnet RPC error type.
pub struct Error(pub MixnetError);

/// Base code for all mixnet errors.
const BASE_ERROR: i32 = crate::error::base::MIXNET;

impl From<Error> for ErrorObjectOwned {
	fn from(err: Error) -> Self {
		let code = match err.0 {
			MixnetError::ServiceUnavailable => BASE_ERROR + 1,
			MixnetError::NoReply => BASE_ERROR + 2,
			MixnetError::BadReply => BASE_ERROR + 3,
			MixnetError::Post(PostErr::TooManyFragments) => BASE_ERROR + 101,
			MixnetError::Post(PostErr::SessionMixnodesNotKnown(_)) => BASE_ERROR + 102,
			MixnetError::Post(PostErr::SessionDisabled(_)) => BASE_ERROR + 103,
			MixnetError::Post(PostErr::Topology(TopologyErr::NoConnectedGatewayMixnodes)) => {
				BASE_ERROR + 151
			},
			MixnetError::Post(PostErr::Topology(_)) => BASE_ERROR + 150,
			MixnetError::Post(_) => BASE_ERROR + 100,
			MixnetError::Remote(RemoteErr::Other(_)) => BASE_ERROR + 200,
			MixnetError::Remote(RemoteErr::Decode(_)) => BASE_ERROR + 201,
		};
		ErrorObject::owned(code, err.0.to_string(), None::<()>)
	}
}
