use alloc::string::String;
use crate::types::{Result, SdkError, Hash};


/// A super-light RPC trait the embedded app must implement or provide.
pub trait RpcClient {
/// POST a JSON string to an RPC URL, returning response body as UTF-8.
fn post_json(&mut self, url: &str, json_body: &str) -> Result<String>;
}


pub struct SimpleRpc<'a, C: RpcClient> {
pub url: &'a str,
pub client: C,
}


impl<'a, C: RpcClient> SimpleRpc<'a, C> {
pub fn new(url: &'a str, client: C) -> Self { Self { url, client } }


pub fn get_latest_blockhash(&mut self) -> Result<Hash> {
let body = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getLatestBlockhash\"}";
let resp = self.client.post_json(self.url, body)?;
// Minimal string search to extract blockhash (host-only path)
#[cfg(feature = "std")]
{
let key = "\"blockhash\":\"";
if let Some(start) = resp.find(key) {
let s = start + key.len();
if let Some(end) = resp[s..].find('"') {
let bh = &resp[s..s + end];
let decoded = bs58::decode(bh).into_vec().map_err(|_| SdkError::Serialize)?;
if decoded.len() != 32 { return Err(SdkError::Invalid); }
let mut arr = [0u8; 32];
arr.copy_from_slice(&decoded);
return Ok(Hash::new(&arr));
}
}
return Err(SdkError::Serialize);
}
#[allow(unreachable_code)]
Err(SdkError::Serialize)
}


pub fn send_transaction_base64(&mut self, b64: &str) -> Result<String> {
let body = alloc::format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"sendTransaction\",\"params\":[\"{}\"]}}", b64);
let resp = self.client.post_json(self.url, &body)?;
#[cfg(feature = "std")]
{
let key = "\"result\":\"";
if let Some(start) = resp.find(key) {
let s = start + key.len();
if let Some(end) = resp[s..].find('"') {
let sig = resp[s..s + end].to_string();
return Ok(sig);
}
}
return Err(SdkError::Serialize);
}
Err(SdkError::Serialize)
}
}


#[cfg(feature = "std")]
pub mod host_impl {
use super::*;
use crate::types::{Result, SdkError};


pub struct UreqClient;
impl RpcClient for UreqClient {
fn post_json(&mut self, url: &str, json_body: &str) -> Result<String> {
let resp = ureq::post(url)
.set("Content-Type", "application/json")
.send_string(json_body)
.map_err(|_| SdkError::Rpc)?;
let text = resp.into_string().map_err(|_| SdkError::Rpc)?;
Ok(text)
}
}
}