use crate::{crypto::Keypair, hash::Hash, prelude::Instruction};

pub struct Transaction<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
where
    'b: 'a,
    'd: 'c,
    'e: 'c,
    'f: 'c,
    'g: 'c,
{
    pub signers: &'a [&'b Keypair],
    pub instructions: &'c [Instruction<'d, 'e, 'f, 'g>],
    pub recent_blockhash: &'h Hash,
}
