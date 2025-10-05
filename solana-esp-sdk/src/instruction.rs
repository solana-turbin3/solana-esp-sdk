use crate::crypto::Pubkey;

#[derive(Debug, Clone)]
pub struct AccountMeta<'a> {
    pub pubkey: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl<'a> AccountMeta<'a> {
    pub fn new_writable(pubkey: &'a Pubkey, is_signer: bool) -> AccountMeta<'a> {
        AccountMeta {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }

    pub fn new_readonly(pubkey: &'a Pubkey, is_signer: bool) -> AccountMeta<'a> {
        AccountMeta {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction<'a, 'b, 'c, 'd>
where
    'a: 'b,
{
    /// Public key of the program.
    pub program_id: &'c Pubkey,

    /// Data expected by the program instruction.
    pub data: &'d [u8],

    /// Metadata describing accounts that should be passed to the program.
    pub accounts: &'b [AccountMeta<'a>],
}
