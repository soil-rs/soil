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
use soil_mixnet::{PostErr, RemoteErr, TopologyErr};

/// Mixnet RPC error type.
pub struct Error(pub soil_mixnet::Error);

/// Base code for all mixnet errors.
const BASE_ERROR: i32 = crate::error::base::MIXNET;

impl From<Error> for ErrorObjectOwned {
	fn from(err: Error) -> Self {
		let code = match err.0 {
			soil_mixnet::Error::ServiceUnavailable => BASE_ERROR + 1,
			soil_mixnet::Error::NoReply => BASE_ERROR + 2,
			soil_mixnet::Error::BadReply => BASE_ERROR + 3,
			soil_mixnet::Error::Post(PostErr::TooManyFragments) => BASE_ERROR + 101,
			soil_mixnet::Error::Post(PostErr::SessionMixnodesNotKnown(_)) => BASE_ERROR + 102,
			soil_mixnet::Error::Post(PostErr::SessionDisabled(_)) => BASE_ERROR + 103,
			soil_mixnet::Error::Post(PostErr::Topology(TopologyErr::NoConnectedGatewayMixnodes)) => {
				BASE_ERROR + 151
			},
			soil_mixnet::Error::Post(PostErr::Topology(_)) => BASE_ERROR + 150,
			soil_mixnet::Error::Post(_) => BASE_ERROR + 100,
			soil_mixnet::Error::Remote(RemoteErr::Other(_)) => BASE_ERROR + 200,
			soil_mixnet::Error::Remote(RemoteErr::Decode(_)) => BASE_ERROR + 201,
		};
		ErrorObject::owned(code, err.0.to_string(), None::<()>)
	}
}
