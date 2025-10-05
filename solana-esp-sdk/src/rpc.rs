use core::future::Future;

use base64::Engine;

use crate::{
    crypto::Pubkey,
    hash::Hash,
    signature::Signature,
    transaction::Transaction,
    types::{Result, SdkError},
};

/// Transport-agnostic RPC trait. Implement this on host or embedded.
/// Implementations perform a blocking POST of raw JSON and return UTF-8 response.
pub trait SyncClient {
    // fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::vec::Vec<u8>>;
    fn post_json<'a>(
        &self,
        url: &str,
        json_body: &[u8],
        resp_buffer: &'a mut [u8],
    ) -> Result<&'a [u8]>;
}

/// Transport-agnostic async RPC trait. Implement this on host or embedded.
/// Implementations perform a async POST of raw JSON and return response.
pub trait AsyncClient {
    // async fn post_json<'a>(&self, url: &str, json_body: &[u8]) -> Result<alloc::vec::Vec<u8>>;
    // fn post_json<'a>(
    //     &self,
    //     url: &str,
    //     json_body: &[u8],
    // ) -> impl Future<Output = Result<alloc::vec::Vec<u8>>>;
    fn post_json<'a>(
        &self,
        url: &str,
        json_body: &[u8],
        resp_buffer: &'a mut [u8],
    ) -> impl Future<Output = Result<&'a [u8]>>;
}

pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

/// Thin helper struct parameterized by an RpcClient impl.
pub struct RpcClient<'a, C> {
    url: &'a str,
    commitment: Commitment,
    client: C,
}

impl<'a, C> RpcClient<'a, C> {
    /// Creates a new `Connection` with a **sync-only** client.
    pub fn new_sync(url: &'a str, commitment: Commitment, client: C) -> Self
    where
        C: SyncClient,
    {
        Self {
            url,
            commitment,
            client,
        }
    }

    /// Creates a new `Connection` with an **async-only** client.
    pub fn new_async(url: &'a str, commitment: Commitment, client: C) -> Self
    where
        C: AsyncClient,
    {
        Self {
            url,
            commitment,
            client,
        }
    }

    fn extract_blockhash(json: &[u8]) -> Result<Hash> {
        // Step 1: Find the `"blockhash":"` prefix
        let blockhash_prefix = br#""blockhash":""#;
        let start = json
            .windows(blockhash_prefix.len())
            .position(|w| w == blockhash_prefix)
            .ok_or(SdkError::ResponseParseError)?
            + blockhash_prefix.len();

        // Step 2: Extract the blockhash (ends with `"`)
        let end = json[start..]
            .iter()
            .position(|&c| c == b'"')
            .ok_or(SdkError::ResponseParseError)?
            + start;

        // Step 3: Decode the blockhash
        let mut hash_bytes = [0u8; 32];
        five8::decode_32(&json[start..end], &mut hash_bytes)
            .map_err(|_| SdkError::ResponseParseError)?;
        Ok(Hash::from(hash_bytes))
    }

    fn extract_signature(json: &[u8]) -> Result<Signature> {
        let result_prefix = br#""result":""#;
        let start = json
            .windows(result_prefix.len())
            .position(|w| w == result_prefix)
            .ok_or(SdkError::ResponseParseError)?
            + result_prefix.len();
        let end = json[start..]
            .iter()
            .position(|&c| c == b'"')
            .ok_or(SdkError::ResponseParseError)?
            + start;
        let mut sig_bytes = [0u8; 64];
        five8::decode_64(&json[start..end], &mut sig_bytes)
            .map_err(|_| SdkError::ResponseParseError)?;
        Ok(Signature::from(sig_bytes))
    }
}

impl<'a, C: AsyncClient> RpcClient<'a, C> {
    pub async fn get_latest_blockhash(&self) -> Result<Hash> {
        let json_body = match self.commitment {
            Commitment::Processed => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "processed"}]}"#,
            Commitment::Confirmed => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "confirmed"}]}"#,
            Commitment::Finalized => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "finalized"}]}"#,
        };
        let mut resp_buffer = [0u8; 4096];
        let reponse = self
            .client
            .post_json(self.url, json_body.as_slice(), resp_buffer.as_mut_slice())
            .await?;
        Self::extract_blockhash(&reponse)
    }

    pub async fn send_transaction(
        &self,
        transaction: &Transaction<'_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Result<Signature> {
        struct KeyMetadata {
            pub is_signer: bool,
            pub is_writable: bool,
        }

        // get map of all accounts
        let mut keys_meta_map: heapless::LinearMap<&Pubkey, KeyMetadata, 35> =
            heapless::LinearMap::new();
        for instruction in transaction.instructions.iter() {
            // program cannot be writable or signer so it is safe to overwrite
            let _ = keys_meta_map.insert(
                &instruction.program_id,
                KeyMetadata {
                    is_signer: false,
                    is_writable: false,
                },
            );
            for account_meta in instruction.accounts.iter() {
                let key_meta = keys_meta_map.get_mut(account_meta.pubkey);
                if let Some(key_meta) = key_meta {
                    key_meta.is_writable |= account_meta.is_writable;
                    key_meta.is_signer |= account_meta.is_signer;
                    continue;
                }
                let _ = keys_meta_map.insert(
                    account_meta.pubkey,
                    KeyMetadata {
                        is_signer: account_meta.is_signer,
                        is_writable: account_meta.is_writable,
                    },
                );
            }
        }

        let mut writable_signer_keys: heapless::Vec<&Pubkey, 35> = heapless::Vec::new();
        let mut readonly_signer_keys: heapless::Vec<&Pubkey, 35> = heapless::Vec::new();
        let mut writable_non_signer_keys: heapless::Vec<&Pubkey, 35> = heapless::Vec::new();
        let mut readonly_non_signer_keys: heapless::Vec<&Pubkey, 35> = heapless::Vec::new();
        let mut static_account_keys: heapless::Vec<&Pubkey, 35> = heapless::Vec::new();

        for (key, meta) in keys_meta_map.iter() {
            if meta.is_writable {
                if meta.is_signer {
                    let _ = writable_signer_keys.push(*key);
                } else {
                    let _ = writable_non_signer_keys.push(*key).ok();
                }
            } else {
                if meta.is_signer {
                    let _ = readonly_signer_keys.push(*key).ok();
                } else {
                    let _ = readonly_non_signer_keys.push(*key).ok();
                }
            }
        }
        let num_required_signatures: u8 =
            (writable_signer_keys.len() + readonly_signer_keys.len()) as u8;
        let num_readonly_signed_accounts: u8 = readonly_signer_keys.len() as u8;
        let num_readonly_unsigned_accounts: u8 = readonly_non_signer_keys.len() as u8;
        static_account_keys.extend(writable_signer_keys.into_iter());
        static_account_keys.extend(readonly_signer_keys.into_iter());
        static_account_keys.extend(writable_non_signer_keys.into_iter());
        static_account_keys.extend(readonly_non_signer_keys.into_iter());

        // build message
        let mut msg_buffer: heapless::Vec<u8, 1200> = heapless::Vec::new(); // can be smaller

        // SAFETY: msg_buffer is empty and has enough space
        unsafe {
            msg_buffer.push_unchecked(num_required_signatures);
            msg_buffer.push_unchecked(num_readonly_signed_accounts);
            msg_buffer.push_unchecked(num_readonly_unsigned_accounts);
            // number of accounts is less then 128, so 1 byte is enough
            msg_buffer.push_unchecked(static_account_keys.len() as u8);
        };

        for key in static_account_keys.iter() {
            // Result is always Ok because msg_buffer has enough space
            let _ = msg_buffer.extend_from_slice(key.as_ref());
        }

        // Result is always Ok because msg_buffer has enough space
        let _ = msg_buffer.extend_from_slice(transaction.recent_blockhash.as_ref());

        // number of instructions is less then 128, so 1 byte is enough
        // SAFETY: msg_buffer has enough space
        unsafe {
            msg_buffer.push_unchecked(transaction.instructions.len() as u8);
        };

        let mut compiled_instruction: heapless::Vec<u8, 1100> = heapless::Vec::new();
        for instruction in transaction.instructions.iter() {
            // find position of program_id in static_account_keys
            let position = static_account_keys
                .iter()
                .position(|k| k == &instruction.program_id)
                .unwrap(); // always exists

            compiled_instruction
                .push(position as u8)
                .map_err(|_| SdkError::TransactionTooLarge)?;

            // number of accounts is less then 128, so 1 byte is enough
            compiled_instruction
                .push(instruction.accounts.len() as u8)
                .map_err(|_| SdkError::TransactionTooLarge)?;

            for account_meta in instruction.accounts.iter() {
                let key = account_meta.pubkey;
                // find position in static_account_keys
                let position = static_account_keys.iter().position(|k| k == &key).unwrap(); // always exists

                compiled_instruction
                    .push(position as u8)
                    .map_err(|_| SdkError::TransactionTooLarge)?;
            }

            // size of data
            let data_len = instruction.data.len();
            if data_len < 128 {
                compiled_instruction
                    .push(data_len as u8)
                    .map_err(|_| SdkError::TransactionTooLarge)?;
            } else {
                compiled_instruction
                    .push(data_len as u8)
                    .map_err(|_| SdkError::TransactionTooLarge)?;
                compiled_instruction
                    .push((data_len >> 7) as u8)
                    .map_err(|_| SdkError::TransactionTooLarge)?;
            }
            compiled_instruction
                .extend_from_slice(&instruction.data)
                .map_err(|_| SdkError::TransactionTooLarge)?;

            msg_buffer
                .extend_from_slice(&compiled_instruction)
                .map_err(|_| SdkError::TransactionTooLarge)?;

            compiled_instruction.clear();
        }

        // sign message
        let mut transaction_bytes: heapless::Vec<u8, 1232> = heapless::Vec::new();

        // SAFETY: sig_buffer is empty
        unsafe {
            transaction_bytes.push_unchecked(transaction.signers.len() as u8);
        }

        for signer in transaction.signers.iter() {
            let signature = signer.sign_message(msg_buffer.as_slice(), None);
            transaction_bytes
                .extend_from_slice(signature.as_ref())
                .map_err(|_| SdkError::TransactionTooLarge)?;
        }

        transaction_bytes
            .extend_from_slice(msg_buffer.as_slice())
            .map_err(|_| SdkError::TransactionTooLarge)?;

        // send transaction
        let mut json_body: heapless::Vec<u8, 4096> = heapless::Vec::new();
        let _ = json_body.extend_from_slice(
            br#"{"jsonrpc":"2.0","id":1,"method":"sendTransaction","params":[""#,
        );
        let transaction_base64_max_len = (transaction_bytes.len() * 4 / 3) + 4;
        let current_len = json_body.len();
        let _ = json_body.resize_default(transaction_base64_max_len + current_len);

        // get slice from empty space
        let mut transaction_base64 =
            &mut json_body[current_len..transaction_base64_max_len + current_len];

        let bytes_written = base64::engine::general_purpose::STANDARD
            .encode_slice(transaction_bytes.as_slice(), &mut transaction_base64)
            .unwrap();
        json_body.truncate(current_len + bytes_written);

        let _ = json_body.push(b'"');
        let _ = json_body.extend_from_slice(br#",{"encoding":"base64"}]}"#);

        let mut resp_buffer = [0u8; 4096];
        let response = self
            .client
            .post_json(self.url, json_body.as_slice(), resp_buffer.as_mut_slice())
            .await?;

        Self::extract_signature(&response)
    }
}

/*
impl<'a, C: SyncClient> Rpc<'a, C> {
    /// Fetch latest blockhash for transaction building.
    pub fn get_latest_blockhash(&self) -> Result<solana_program::hash::Hash> { /* TODO */ }

    /// Submit a base64-encoded transaction; return signature string.
    pub fn send_transaction_base64(&self, b64_tx: &str) -> Result<alloc::string::String> { /* TODO */ }

    /// Read account lamports only (lighter than full getAccountInfo parse).
    pub fn get_balance(&self, pubkey_b58: &str) -> Result<u64> { /* TODO */ }

    /// Read full account info (owner, lamports, data base64) – heavier path.
    pub fn get_account_info_raw(&self, pubkey_b58: &str) -> Result<alloc::string::String> { /* TODO */ }
}
*/
/*
/// Build the JSON for getLatestBlockhash (optionally with a commitment param).
// pub fn json_get_latest_blockhash() -> alloc::string::String { /* TODO */
// }

/// Build the JSON for sendTransaction (b64 payload).
// pub fn json_send_transaction(b64: &str) -> alloc::string::String { /* TODO */
// }

/// Build the JSON for getBalance(pubkey).
// pub fn json_get_balance(pubkey_b58: &str) -> alloc::string::String { /* TODO */
// }

/// (std) Parse getLatestBlockhash response to 32-byte Hash.
#[cfg(feature = "std")]
pub fn parse_blockhash_from_response(json: &str) -> Result<solana_program::hash::Hash> {
    /* TODO */
}

/// (std) Parse getBalance response to u64.
#[cfg(feature = "std")]
pub fn parse_balance_from_response(json: &str) -> Result<u64> { /* TODO */
}
 */
