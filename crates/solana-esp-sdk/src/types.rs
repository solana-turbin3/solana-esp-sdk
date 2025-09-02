use core::fmt;
use alloc::vec::Vec;


#[derive(Clone, Copy)]
pub struct Signature(pub [u8; 64]);


impl fmt::Display for Signature {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
for b in &self.0 { write!(f, "{:02x}", b)?; }
Ok(())
}
}


#[derive(Debug)]
pub enum SdkError {
Crypto,
Rpc,
Serialize,
Invalid,
}


pub type Result<T> = core::result::Result<T, SdkError>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pubkey(pub [u8; 32]);

impl Pubkey {
pub fn new_from_array(bytes: [u8; 32]) -> Self { Self(bytes) }
pub fn as_ref(&self) -> &[u8; 32] { &self.0 }
}

#[derive(Clone, Copy)]
pub struct Hash(pub [u8; 32]);

impl Hash {
pub fn new(bytes: &[u8; 32]) -> Self { Self(*bytes) }
pub fn as_ref(&self) -> &[u8; 32] { &self.0 }
}

pub type Blockhash = Hash; // alias

#[derive(Clone)]
pub struct MessageHeader {
pub num_required_signatures: u8,
pub num_readonly_signed_accounts: u8,
pub num_readonly_unsigned_accounts: u8,
}

#[derive(Clone)]
pub struct CompiledInstruction {
pub program_id_index: u8,
pub accounts: Vec<u8>,
pub data: Vec<u8>,
}