//! Minimal, no_std-friendly crypto module for Solana on ESP.
//!
//! - Ed25519 keypair generation and signing
//! - Deterministic and randomized (noise) signatures
//! - Verify helper
//! - Optional base58 helpers under `std` (for host-side debugging)

#![allow(clippy::needless_return)]

use rand_core::{CryptoRng, RngCore};
use crate::types::Pubkey;

use crate::types::{Result, SdkError, Signature};

// Compact, no_std-friendly Ed25519 implementation.
use ed25519_compact as ed25519;

/// Wrapper around an Ed25519 keypair suitable for `no_std` use.
///
/// ⚠️ For production, consider storing the secret key in a secure element
/// or encrypting it at rest. This struct keeps the key in RAM.
pub struct Keypair(ed25519::KeyPair);

impl Keypair {
    /// Generate a new keypair from an RNG (HAL RNG on device; `OsRng` on host).
    pub fn generate<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let seed = ed25519::Seed::new(seed);
        Self(ed25519::KeyPair::from_seed(seed))
    }

    /// Recreate a keypair from a 32-byte seed.
    ///
    /// Note: this is *not* the same as importing a 64-byte secret key.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let seed = ed25519::Seed::new(seed);
        Self(ed25519::KeyPair::from_seed(seed))
    }

    /// Return the Solana `Pubkey` (32-byte Ed25519 public key).
    pub fn pubkey(&self) -> Pubkey {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(self.0.pk.as_ref());
        Pubkey::new_from_array(arr)
    }

    /// Deterministic RFC8032 signature over the full message bytes.
    ///
    /// Solana signs the *message bytes* directly (not a pre-hash).
    pub fn sign(&self, msg: &[u8]) -> Signature {
        // Deterministic: no additional noise.
        let sig = self.0.sk.sign(msg, None);
        let mut out = [0u8; 64];
        out.copy_from_slice(sig.as_ref());
        Signature(out)
    }

    /// Randomized signature variant. Supplies per-signature noise for side-channel hardening.
    pub fn sign_with_rng<R: RngCore + CryptoRng>(&self, msg: &[u8], rng: &mut R) -> Signature {
        // 16 bytes of noise for ed25519-compact.
        let mut n = [0u8; 16];
        rng.fill_bytes(&mut n);
        let noise = ed25519::Noise::new(n);
        let sig = self.0.sk.sign(msg, Some(noise));
        let mut out = [0u8; 64];
        out.copy_from_slice(sig.as_ref());
        Signature(out)
    }
}

/// Verify a signature with a given public key.
pub fn verify(pubkey: Pubkey, msg: &[u8], sig: &Signature) -> bool {
    if let Ok(pk) = ed25519::PublicKey::from_slice(pubkey.as_ref()) {
        if let Ok(esig) = ed25519::Signature::from_slice(&sig.0) {
            return pk.verify(msg, &esig).is_ok();
        }
    }
    false
}

/// Serialize a `Pubkey` to base58 (host-only helper for debugging).
#[cfg(feature = "std")]
pub fn pubkey_to_base58(pk: &Pubkey) -> alloc::string::String {
    bs58::encode(pk.as_ref()).into_string()
}

/// Parse a base58 string into a 32-byte array (host-only helper).
#[cfg(feature = "std")]
pub fn base58_to_32(s: &str) -> Result<[u8; 32]> {
    let v = bs58::decode(s).into_vec().map_err(|_| SdkError::Serialize)?;
    if v.len() != 32 {
        return Err(SdkError::Invalid);
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&v);
    Ok(out)
}