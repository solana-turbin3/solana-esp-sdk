use crate::crypto::Address;

#[derive(Debug, Clone)]
pub struct AccountMeta<'a> {
    pub address: &'a Address,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl<'a> AccountMeta<'a> {
    pub fn new_writable(address: &'a Address, is_signer: bool) -> AccountMeta<'a> {
        AccountMeta {
            address,
            is_signer,
            is_writable: true,
        }
    }

    pub fn new_readonly(address: &'a Address, is_signer: bool) -> AccountMeta<'a> {
        AccountMeta {
            address,
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
    pub program_id: &'c Address,

    /// Data expected by the program instruction.
    pub data: &'d [u8],

    /// Metadata describing accounts that should be passed to the program.
    pub accounts: &'b [AccountMeta<'a>],
}
