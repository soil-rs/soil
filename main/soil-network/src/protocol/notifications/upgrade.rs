// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

#[cfg(test)]
pub(crate) use self::notifications::{
	NotificationsInOpen, NotificationsInSubstreamHandshake, NotificationsOutOpen,
};

pub(crate) use notifications::NotificationsOutError;

pub use self::{
	collec::UpgradeCollec,
	notifications::{
		NotificationsIn, NotificationsInSubstream, NotificationsOut, NotificationsOutSubstream,
	},
};

mod collec;
mod notifications;
