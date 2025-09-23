#[cfg(feature = "net-reqwless")]
use embedded_nal_async::{Dns, TcpConnect};
#[cfg(feature = "net-reqwless")]
use reqwless::{
    client::{HttpClient, TlsConfig},
    headers::ContentType,
    request::RequestBuilder,
};

use crate::rpc::AsyncClient;
use crate::types::{Result, SdkError};

/// A reqwless-based RpcAsyncClient (no_std HTTP client for embedded).
#[cfg(feature = "net-reqwless")]
pub struct ReqwlessAsyncClient<T, D>
where
    T: TcpConnect,
    D: Dns,
{
    pub tcp: T,
    pub dns: D,
    pub tls_seed: u64,
}

impl<T, D> AsyncClient for ReqwlessAsyncClient<T, D>
where
    T: TcpConnect,
    D: Dns,
{
    async fn post_json<'a>(&self, url: &str, json_body: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let tls = TlsConfig::new(
            self.tls_seed,
            &mut rx_buffer,
            &mut tx_buffer,
            reqwless::client::TlsVerify::None,
        );

        let mut client = HttpClient::new_with_tls(&self.tcp, &self.dns, tls);
        let mut buffer = [0u8; 4096];
        let mut http_req = client
            .request(reqwless::request::Method::POST, url)
            .await
            .unwrap()
            .body(json_body)
            .content_type(ContentType::ApplicationJson);
        let response = http_req.send(&mut buffer).await.unwrap();

        let res = response.body().read_to_end().await.unwrap();
        Ok(res.to_vec())
    }
}

/// A smoltcp + manual HTTP/1.1 POST client for ultra-small builds.
#[cfg(feature = "net-smoltcp")]
pub struct SmolHttpClient {/* socket refs, buffers */}

#[cfg(feature = "net-smoltcp")]
impl RpcClient for SmolHttpClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::string::String> {
        /* TODO */
    }
}
