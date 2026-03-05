// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module offers elliptic curves types which are compatible with the
//! [Arkworks](https://github.com/arkworks-rs) library functionalities.
//!
//! The implementation has been primarily designed to be used in slow hosted
//! targets (e.g. wasm32) and offloads the most computationally expensive
//! operations to the host by leveraging the
//! [arkworks-extensions](https://github.com/paritytech/arkworks-extensions)
//! library and Substrate's host functions.
//!
//! The exported types are organized and named in a way that mirrors the structure
//! of the types in the original Arkworks library. This design choice aims to make
//! it easier for users already familiar with the library to understand and utilize
//! the exported types effectively.

#[cfg(feature = "crypto-ec-utils-bls12-377")]
pub mod bls12_377;
#[cfg(feature = "crypto-ec-utils-bls12-381")]
pub mod bls12_381;
#[cfg(feature = "crypto-ec-utils-bw6-761")]
pub mod bw6_761;
#[cfg(feature = "crypto-ec-utils-ed-on-bls12-377")]
pub mod ed_on_bls12_377;
#[cfg(feature = "crypto-ec-utils-ed-on-bls12-381-bandersnatch")]
pub mod ed_on_bls12_381_bandersnatch;

#[cfg(any(
	feature = "crypto-ec-utils-bls12-377",
	feature = "crypto-ec-utils-bls12-381",
	feature = "crypto-ec-utils-bw6-761",
	feature = "crypto-ec-utils-ed-on-bls12-377",
	feature = "crypto-ec-utils-ed-on-bls12-381-bandersnatch",
))]
pub(crate) mod utils;
