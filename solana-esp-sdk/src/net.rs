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
pub struct ReqwlessAsyncClient<T: TcpConnect, D: Dns> {
    pub tcp: T,
    pub dns: D,
    pub tls_seed: u64,
}

#[cfg(feature = "net-reqwless")]
impl<T: TcpConnect, D: Dns> AsyncClient for ReqwlessAsyncClient<T, D> {
    async fn post_json<'a>(
        &self,
        url: &str,
        json_body: &[u8],
        resp_buffer: &'a mut [u8],
    ) -> Result<&'a [u8]> {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        let tls = TlsConfig::new(
            self.tls_seed,
            &mut rx_buffer,
            &mut tx_buffer,
            reqwless::client::TlsVerify::None,
        );

        let mut client = HttpClient::new_with_tls(&self.tcp, &self.dns, tls);
        let mut http_req = client
            .request(reqwless::request::Method::POST, url)
            .await
            .map_err(|_| SdkError::NetworkError)?
            .body(json_body)
            .content_type(ContentType::ApplicationJson);
        let response = http_req
            .send(resp_buffer)
            .await
            .map_err(|_| SdkError::NetworkError)?;

        let res = response
            .body()
            .read_to_end()
            .await
            .map_err(|_| SdkError::NetworkError)?;
        Ok(res)
    }
}

/*
/// A smoltcp + manual HTTP/1.1 POST client for ultra-small builds.
#[cfg(feature = "net-smoltcp")]
pub struct SmolHttpClient {/* socket refs, buffers */}

#[cfg(feature = "net-smoltcp")]
impl RpcClient for SmolHttpClient {
    fn post_json(&self, url: &str, json_body: &str) -> Result<alloc::string::String> {
        /* TODO */
    }
}
*/
