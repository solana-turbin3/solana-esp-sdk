use alloc::{vec, vec::Vec};
use crate::types::Pubkey;


pub struct AccountMeta {
pub pubkey: Pubkey,
pub is_signer: bool,
pub is_writable: bool,
}

pub struct Instruction {
pub program_id: Pubkey,
pub accounts: Vec<AccountMeta>,
pub data: Vec<u8>,
}

pub fn system_program_id() -> Pubkey { Pubkey::new_from_array([0; 32]) }

pub fn system_transfer(from: Pubkey, to: Pubkey, lamports: u64) -> Instruction {
// SystemProgram::Transfer layout: 4 bytes for enum variant + 8 bytes lamports
// For now we build via solana_program helper-free (manual data)
let mut data = Vec::with_capacity(12);
// variant = 2 (Transfer) in little-endian u32
data.extend_from_slice(&2u32.to_le_bytes());
data.extend_from_slice(&lamports.to_le_bytes());


Instruction {
program_id: system_program_id(),
accounts: vec![
AccountMeta { pubkey: from, is_signer: true, is_writable: true },
AccountMeta { pubkey: to, is_signer: false, is_writable: true },
],
data,
}
}


#[cfg(feature = "spl-token")]
pub mod spl_token_helpers {
use super::*;
    /// SPL Token Program id on all clusters
    pub const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    /// Minimal helper for TokenInstruction::Transfer { amount }
    pub fn spl_token_transfer(source: Pubkey, destination: Pubkey, owner: Pubkey, amount: u64) -> Instruction {
    // TokenInstruction::Transfer variant = 3 (as of SPL-Token v3)
    let mut data = Vec::with_capacity(9);
    data.push(3u8);
    data.extend_from_slice(&amount.to_le_bytes());
        Instruction {
        program_id: SPL_TOKEN_PROGRAM_ID,
        accounts: vec![
        AccountMeta::new(source, false),
        AccountMeta::new(destination, false),
        AccountMeta::new_readonly(owner, true),
        ],
        data,
        }
    }
}