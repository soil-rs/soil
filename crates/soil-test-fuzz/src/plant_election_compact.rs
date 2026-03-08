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

use honggfuzz::fuzz;
use plant_election_provider_macros::generate_solution_type;
use subsoil::arithmetic::Percent;
use subsoil::runtime::codec::{Encode, Error};

fn main() {
	generate_solution_type!(
		#[compact] pub struct InnerTestSolutionCompact::<
		VoterIndex = u32,
		TargetIndex = u32,
		Accuracy = Percent,
		MaxVoters = topsoil_support::traits::ConstU32::<100_000>,
	>(16));
	loop {
		fuzz!(|fuzzer_data: &[u8]| {
			let result_decoded: Result<InnerTestSolutionCompact, Error> =
				<InnerTestSolutionCompact as codec::Decode>::decode(&mut &*fuzzer_data);
			if let Ok(decoded) = result_decoded {
				let reencoded: std::vec::Vec<u8> = decoded.encode();
				if fuzzer_data.len() < reencoded.len() {
					panic!("fuzzer_data.len() < reencoded.len()");
				}
				let decoded2: InnerTestSolutionCompact =
					<InnerTestSolutionCompact as codec::Decode>::decode(&mut reencoded.as_slice())
						.unwrap();
				assert_eq!(decoded, decoded2);
			}
		});
	}
}
