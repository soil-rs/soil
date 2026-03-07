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
