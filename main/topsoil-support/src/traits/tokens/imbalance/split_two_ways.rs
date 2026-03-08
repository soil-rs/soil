// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

//! Means for splitting an imbalance into two and handling them differently.

use super::super::imbalance::{Imbalance, OnUnbalanced};
use core::{marker::PhantomData, ops::Div};
use subsoil::runtime::traits::Saturating;

/// Split an unbalanced amount two ways between a common divisor.
pub struct SplitTwoWays<Balance, Imbalance, Target1, Target2, const PART1: u32, const PART2: u32>(
	PhantomData<(Balance, Imbalance, Target1, Target2)>,
);

impl<
		Balance: From<u32> + Saturating + Div<Output = Balance>,
		I: Imbalance<Balance>,
		Target1: OnUnbalanced<I>,
		Target2: OnUnbalanced<I>,
		const PART1: u32,
		const PART2: u32,
	> OnUnbalanced<I> for SplitTwoWays<Balance, I, Target1, Target2, PART1, PART2>
{
	fn on_nonzero_unbalanced(amount: I) {
		let total: u32 = PART1 + PART2;
		let amount1 = amount.peek().saturating_mul(PART1.into()) / total.into();
		let (imb1, imb2) = amount.split(amount1);
		Target1::on_unbalanced(imb1);
		Target2::on_unbalanced(imb2);
	}
}
