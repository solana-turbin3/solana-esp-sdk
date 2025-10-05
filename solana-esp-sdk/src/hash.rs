use core::{
    fmt,
    str::{from_utf8_unchecked, FromStr},
};

/// Size of a hash in bytes.
pub const HASH_BYTES: usize = 32;
/// Maximum string length of a base58 encoded hash.
pub const MAX_BASE58_LEN: usize = 44;

#[derive(Default)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

impl From<[u8; HASH_BYTES]> for Hash {
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_as_base58(f, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHashError {
    WrongSize,
    Invalid,
}

impl FromStr for Hash {
    type Err = ParseHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use five8::DecodeError;
        if s.len() > MAX_BASE58_LEN {
            return Err(ParseHashError::WrongSize);
        }
        let mut bytes = [0; HASH_BYTES];
        five8::decode_32(s, &mut bytes).map_err(|e| match e {
            DecodeError::InvalidChar(_) => ParseHashError::Invalid,
            DecodeError::TooLong
            | DecodeError::TooShort
            | DecodeError::LargestTermTooHigh
            | DecodeError::OutputTooLong => ParseHashError::WrongSize,
        })?;
        Ok(Self::from(bytes))
    }
}

fn write_as_base58(f: &mut fmt::Formatter, h: &Hash) -> fmt::Result {
    let mut out = [0u8; MAX_BASE58_LEN];
    let len = five8::encode_32(&h.0, &mut out) as usize;
    // any sequence of base58 chars is valid utf8
    let as_str = unsafe { from_utf8_unchecked(&out[..len]) };
    f.write_str(as_str)
}
