use solana_program::{
    hash::Hash, instruction::CompiledInstruction, message::MessageHeader, pubkey::Pubkey
};

/// In-memory representation of a Solana Message (pre-transaction).
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: alloc::vec::Vec<Pubkey>,
    pub recent_blockhash: Hash,
    pub instructions: alloc::vec::Vec<CompiledInstruction>,
}

/// Compute canonical account list + header from high-level instructions.
/// - De-duplicate pubkeys, put signers first, etc.
pub fn compile_message(
    payer: Pubkey,
    ixs: &[solana_program::instruction::Instruction],
    recent_blockhash: Hash,
) -> Result<Message> { /* TODO */ }

/// Serialize a `Message` to exact Solana wire format bytes.
pub fn serialize_message(msg: &Message) -> Result<alloc::vec::Vec<u8>> { /* TODO */ }