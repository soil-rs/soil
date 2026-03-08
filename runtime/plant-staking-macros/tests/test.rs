// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Test crate for topsoil-staking-reward-curve. Allows to test for procedural macro.
//! See tests directory.

mod test_small_falloff {
	plant_staking_macros::build! {
		const REWARD_CURVE: subsoil::runtime::curve::PiecewiseLinear<'static> = curve!(
			min_inflation: 0_020_000,
			max_inflation: 0_200_000,
			ideal_stake: 0_600_000,
			falloff: 0_010_000,
			max_piece_count: 200,
			test_precision: 0_005_000,
			);
	}
}

mod test_big_falloff {
	plant_staking_macros::build! {
		const REWARD_CURVE: subsoil::runtime::curve::PiecewiseLinear<'static> = curve!(
			min_inflation: 0_100_000,
			max_inflation: 0_400_000,
			ideal_stake: 0_400_000,
			falloff: 1_000_000,
			max_piece_count: 40,
			test_precision: 0_005_000,
			);
	}
}
