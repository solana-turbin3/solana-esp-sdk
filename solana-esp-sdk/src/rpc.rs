use core::future::Future;

use crate::{
    hash::Hash,
    types::{Result, SdkError},
};

/// Transport-agnostic RPC trait. Implement this on host or embedded.
/// Implementations perform a blocking POST of raw JSON and return UTF-8 response.
pub trait SyncClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::vec::Vec<u8>>;
}

/// Transport-agnostic async RPC trait. Implement this on host or embedded.
/// Implementations perform a async POST of raw JSON and return response.
pub trait AsyncClient {
    // async fn post_json<'a>(&self, url: &str, json_body: &[u8]) -> Result<alloc::vec::Vec<u8>>;
    fn post_json<'a>(&self, url: &str, json_body: &[u8]) -> impl Future<Output = Result<alloc::vec::Vec<u8>>>;
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

    fn extract_blockhash(json: &[u8]) -> Option<Hash> {
        // Step 1: Find the `"blockhash":"` prefix
        let blockhash_prefix = b"\"blockhash\":\"";
        let start = json
            .windows(blockhash_prefix.len())
            .position(|w| w == blockhash_prefix)?
            + blockhash_prefix.len();

        // Step 2: Extract the blockhash (ends with `"`)
        let end = json[start..].iter().position(|&c| c == b'"')? + start;

        // Step 3: Return the slice as a str
        // Hash::from_str(&json[start..end]).ok()
        let mut hash_bytes = [0u8; 32];
        five8::decode_32(&json[start..end], &mut hash_bytes).unwrap(); // TODO: Propagate error
        Some(Hash::from(hash_bytes))
    }
}

impl<'a, C: AsyncClient> RpcClient<'a, C> {
    pub async fn get_latest_blockhash(&self) -> Result<Hash> {
        let json_body = match self.commitment {
            Commitment::Processed => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "processed"}]}"#,
            Commitment::Confirmed => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "confirmed"}]}"#,
            Commitment::Finalized => br#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash","params":[{"commitment": "finalized"}]}"#,
        };
        let reponse = self
            .client
            .post_json(self.url, json_body.as_slice())
            .await?;
        let hash = Self::extract_blockhash(&reponse).unwrap(); // TODO: Propagate error
        Ok(hash)
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
