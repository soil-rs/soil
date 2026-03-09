// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

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
		MaxVoters = topsoil_core::traits::ConstU32::<100_000>,
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
