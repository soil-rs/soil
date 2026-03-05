#![cfg_attr(not(feature = "std"), no_std)]

pub use subsoil_crypto_hashing as crypto_hashing;

#[allow(clippy::module_inception)]
pub mod std;
#[cfg(feature = "std")]
pub mod panic_handler;
