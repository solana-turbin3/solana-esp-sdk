use alloc::vec::Vec;
use crate::types::{Hash, CompiledInstruction, MessageHeader, Pubkey};


use crate::types::Result;


pub struct Message {
pub header: MessageHeader,
pub account_keys: Vec<Pubkey>,
pub recent_blockhash: Hash,
pub instructions: Vec<CompiledInstruction>,
}


impl Message {
pub fn new(header: MessageHeader, account_keys: Vec<Pubkey>, recent_blockhash: Hash, instructions: Vec<CompiledInstruction>) -> Self {
Self { header, account_keys, recent_blockhash, instructions }
}
}