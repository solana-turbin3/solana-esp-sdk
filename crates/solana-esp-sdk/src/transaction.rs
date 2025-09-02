use alloc::{vec, vec::Vec};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use crate::types::{Hash, CompiledInstruction, MessageHeader, Pubkey};

use crate::{
crypto::Keypair,
message::Message,
types::{Result, Signature},
};

pub struct Transaction {
pub message: Message,
pub signatures: Vec<Signature>,
}

impl Transaction {
pub fn compile(message: Message, signers: &[&Keypair]) -> Result<Self> {
let msg_bytes = serialize_message(&message)?;
let mut signatures = Vec::with_capacity(signers.len());
for kp in signers { signatures.push(kp.sign(&msg_bytes)); }
Ok(Self { message, signatures })
}

pub fn to_base64(&self) -> Result<alloc::string::String> {
let tx_bytes = serialize_transaction(self)?;
Ok(BASE64.encode(tx_bytes))
}
}

fn serialize_message(message: &Message) -> Result<Vec<u8>> {
// Minimal encoder: header, account keys, recent_blockhash, compiled instructions
// NOTE: This is a placeholder — in real code, match Solana wire format exactly.
let mut out = Vec::new();
out.push(message.header.num_required_signatures);
out.push(message.header.num_readonly_signed_accounts);
out.push(message.header.num_readonly_unsigned_accounts);


// account keys
write_uleb128(&mut out, message.account_keys.len() as u64);
for k in &message.account_keys { out.extend_from_slice(k.as_ref()); }


// recent blockhash
out.extend_from_slice(message.recent_blockhash.as_ref());


// instructions
write_uleb128(&mut out, message.instructions.len() as u64);
for ix in &message.instructions {
out.push(ix.program_id_index);
write_uleb128(&mut out, ix.accounts.len() as u64);
out.extend_from_slice(&ix.accounts);
write_uleb128(&mut out, ix.data.len() as u64);
out.extend_from_slice(&ix.data);
}


Ok(out)
}


fn serialize_transaction(tx: &Transaction) -> Result<Vec<u8>> {
let mut out = Vec::new();
// signature count
write_uleb128(&mut out, tx.signatures.len() as u64);
for sig in &tx.signatures { out.extend_from_slice(&sig.0); }
// message bytes
let msg = serialize_message(&tx.message)?;
out.extend_from_slice(&msg);
Ok(out)
}


fn write_uleb128(buf: &mut Vec<u8>, mut v: u64) { while v >= 0x80 { buf.push((v as u8) | 0x80); v >>= 7; } buf.push(v as u8); }


pub fn compile_simple_transfer(payer: &Keypair, to: Pubkey, lamports: u64, recent_blockhash: Hash) -> Result<Transaction> {
// accounts: payer, to, system_program
let account_keys = vec![payer.pubkey(), to, crate::instruction::system_program_id()];
let header = MessageHeader { num_required_signatures: 1, num_readonly_signed_accounts: 0, num_readonly_unsigned_accounts: 1 };


// Build instruction and compile indices
let ix = crate::instruction::system_transfer(account_keys[0], account_keys[1], lamports);
let compiled = CompiledInstruction {
program_id_index: 2, // system_program at index 2
accounts: alloc::vec![0, 1],
data: ix.data,
};


let msg = crate::message::Message::new(header, account_keys, recent_blockhash, alloc::vec![compiled]);
Transaction::compile(msg, &[payer])
}