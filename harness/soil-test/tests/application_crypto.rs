// This file is part of Soil.

// Copyright (C) Soil contributors.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

#[cfg(feature = "bls-experimental")]
#[path = "application_crypto/bls381.rs"]
mod bls381;
#[path = "application_crypto/ecdsa.rs"]
mod ecdsa;
#[cfg(feature = "bls-experimental")]
#[path = "application_crypto/ecdsa_bls381.rs"]
mod ecdsa_bls381;
#[path = "application_crypto/ed25519.rs"]
mod ed25519;
#[path = "application_crypto/sr25519.rs"]
mod sr25519;
