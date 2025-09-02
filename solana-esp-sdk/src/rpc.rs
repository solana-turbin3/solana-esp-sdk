use crate::types::{Result, SdkError};

/// Transport-agnostic RPC trait. Implement this on host or embedded.
/// Implementations perform a blocking POST of raw JSON and return UTF-8 response.
pub trait RpcClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::string::String>;
}

/// Thin helper struct parameterized by an RpcClient impl.
pub struct Rpc<'a, C: RpcClient> {
    pub url: &'a str,
    pub client: C,
}

impl<'a, C: RpcClient> Rpc<'a, C> {
    /// Fetch latest blockhash for transaction building.
    pub fn get_latest_blockhash(&self) -> Result<solana_program::hash::Hash> { /* TODO */ }

    /// Submit a base64-encoded transaction; return signature string.
    pub fn send_transaction_base64(&self, b64_tx: &str) -> Result<alloc::string::String> { /* TODO */ }

    /// Read account lamports only (lighter than full getAccountInfo parse).
    pub fn get_balance(&self, pubkey_b58: &str) -> Result<u64> { /* TODO */ }

    /// Read full account info (owner, lamports, data base64) – heavier path.
    pub fn get_account_info_raw(&self, pubkey_b58: &str) -> Result<alloc::string::String> { /* TODO */ }
}

/// Build the JSON for getLatestBlockhash (optionally with a commitment param).
pub fn json_get_latest_blockhash() -> alloc::string::String { /* TODO */ }

/// Build the JSON for sendTransaction (b64 payload).
pub fn json_send_transaction(b64: &str) -> alloc::string::String { /* TODO */ }

/// Build the JSON for getBalance(pubkey).
pub fn json_get_balance(pubkey_b58: &str) -> alloc::string::String { /* TODO */ }

/// (std) Parse getLatestBlockhash response to 32-byte Hash.
#[cfg(feature = "std")]
pub fn parse_blockhash_from_response(json: &str) -> Result<solana_program::hash::Hash> { /* TODO */ }

/// (std) Parse getBalance response to u64.
#[cfg(feature = "std")]
pub fn parse_balance_from_response(json: &str) -> Result<u64> { /* TODO */ }