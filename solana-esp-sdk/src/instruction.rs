use solana_program::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

/// System Program: create a simple lamport transfer instruction.
/// Accounts: [from (signer, writable), to (writable)]
pub fn system_transfer(from: Pubkey, to: Pubkey, lamports: u64) -> Instruction { /* TODO */ }

/// Memo Program (optional feature): attach a human string to a tx.
#[cfg(feature = "memo")]
pub fn memo(text: &str) -> Instruction { /* TODO */ }