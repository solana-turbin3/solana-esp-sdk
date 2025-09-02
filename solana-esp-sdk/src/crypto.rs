/// Generate a keypair using device RNG (HAL on ESP, OsRng on host).
pub fn keypair_generate<R: rand_core::RngCore + rand_core::CryptoRng>(rng: &mut R) -> Keypair { /* TODO */ }

/// Construct a keypair from a 32-byte seed (deterministic).
pub fn keypair_from_seed(seed: [u8; 32]) -> Keypair { /* TODO */ }

/// Return the Solana Pubkey for a keypair.
pub fn keypair_pubkey(kp: &Keypair) -> solana_program::pubkey::Pubkey { /* TODO */ }

/// Ed25519 sign raw message bytes (Solana signs the full message).
pub fn sign_message(kp: &Keypair, msg: &[u8]) -> Signature { /* TODO */ }

/// Verify an Ed25519 signature against a message and pubkey.
pub fn verify_signature(pk: &solana_program::pubkey::Pubkey, msg: &[u8], sig: &Signature) -> bool { /* TODO */ }

/// (Host-only) Convert a pubkey to base58 for debugging.
#[cfg(feature = "std")]
pub fn debug_pubkey_b58(pk: &solana_program::pubkey::Pubkey) -> alloc::string::String { /* TODO */ }