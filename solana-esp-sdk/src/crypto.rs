use core::ops::Deref;

use ed25519_compact::{KeyPair as Ed25519CompactKeyPair, Noise, PublicKey, Seed, Signature};

#[derive(Debug)]
#[repr(transparent)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new(pubkey: [u8; 32]) -> Pubkey {
        Pubkey(pubkey)
    }

    pub fn verify_signature(&self, message: impl AsRef<[u8]>, sig: &[u8; 64]) -> bool {
        PublicKey::new(self.0)
            .verify(message, &Signature::new(*sig))
            .is_ok()
    }
}

impl AsRef<[u8; 32]> for Pubkey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Deref for Pubkey {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Pubkey> for [u8; 32] {
    fn as_ref(&self) -> &Pubkey {
        // SAFETY: Pubkey is a newtype around [u8; 32], so this is safe.
        unsafe { &*(self as *const [u8; 32] as *const Pubkey) }
    }
}

pub struct Keypair(Ed25519CompactKeyPair);

impl Keypair {
    pub fn new_from_array(secret_key: [u8; 64]) -> Keypair {
        // unwrap is safe because pk is 64 bytes
        Keypair(Ed25519CompactKeyPair::from_slice(secret_key.as_slice()).unwrap())
    }

    pub fn new_from_seed(seed: [u8; 32]) -> Keypair {
        Keypair(Ed25519CompactKeyPair::from_seed(Seed::from(seed)))
    }

    // pub fn public_key(&self) -> &[u8; 32] {
    //     &self.0.pk
    // }

    pub fn public_key(&self) -> &Pubkey {
        self.0.pk.as_ref()
    }

    pub fn secret_key(&self) -> &[u8; 64] {
        &self.0.sk
    }
    pub fn sign_message(&self, message: impl AsRef<[u8]>, noise: Option<[u8; 16]>) -> [u8; 64] {
        *self.0.sk.sign(message, noise.map(|n| Noise::new(n)))
    }
}

/*
/// Generate a keypair using device RNG (HAL on ESP, OsRng on host).
pub fn keypair_generate<R: rand_core::RngCore + rand_core::CryptoRng>(rng: &mut R) -> Keypair {
    /* TODO */
}

/// (Host-only) Convert a pubkey to base58 for debugging.
#[cfg(feature = "std")]
pub fn debug_pubkey_b58(pk: &solana_program::pubkey::Pubkey) -> alloc::string::String {
    /* TODO */
}
*/
