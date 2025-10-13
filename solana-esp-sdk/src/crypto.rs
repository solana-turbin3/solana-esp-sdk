use core::{fmt, ops::Deref, str::from_utf8_unchecked};

use ed25519_compact::{KeyPair as Ed25519CompactKeyPair, Noise, PublicKey, Seed, Signature};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Address([u8; 32]);

impl Address {
    pub fn new(address: [u8; 32]) -> Address {
        Address(address)
    }

    pub fn verify_signature(&self, message: impl AsRef<[u8]>, sig: &[u8; 64]) -> bool {
        PublicKey::new(self.0)
            .verify(message, &Signature::new(*sig))
            .is_ok()
    }
}

impl AsRef<[u8; 32]> for Address {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Deref for Address {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Address> for [u8; 32] {
    fn as_ref(&self) -> &Address {
        // SAFETY: Address is a newtype around [u8; 32], so this is safe.
        unsafe { &*(self as *const [u8; 32] as *const Address) }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

fn write_as_base58(f: &mut fmt::Formatter, h: &Address) -> fmt::Result {
    let mut out = [0u8; 44];
    let len = five8::encode_32(&h.0, &mut out) as usize;
    // any sequence of base58 chars is valid utf8
    let as_str = unsafe { from_utf8_unchecked(&out[..len]) };
    f.write_str(as_str)
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

    pub fn public_key(&self) -> &Address {
        self.0.pk.as_ref()
    }

    pub fn secret_key(&self) -> &[u8; 64] {
        &self.0.sk
    }
    pub fn sign_message(&self, message: impl AsRef<[u8]>, noise: Option<[u8; 16]>) -> [u8; 64] {
        *self.0.sk.sign(message, noise.map(|n| Noise::new(n)))
    }
}
