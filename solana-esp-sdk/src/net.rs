use crate::types::{Result, SdkError};
use crate::rpc::RpcClient;

/// A reqwless-based RpcClient (no_std HTTP client for embedded).
#[cfg(feature = "net-reqwless")]
pub struct ReqwlessClient { /* sockets, tls, buffers */ }

#[cfg(feature = "net-reqwless")]
impl RpcClient for ReqwlessClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::string::String> { /* TODO */ }
}

/// A smoltcp + manual HTTP/1.1 POST client for ultra-small builds.
#[cfg(feature = "net-smoltcp")]
pub struct SmolHttpClient { /* socket refs, buffers */ }

#[cfg(feature = "net-smoltcp")]
impl RpcClient for SmolHttpClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::string::String> { /* TODO */ }
}