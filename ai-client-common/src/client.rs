use std::time::Duration;

use reqwest::{
  header::{AsHeaderName, HeaderMap, HeaderName, HeaderValue}, Client,
  Proxy,
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

pub struct HttpClientOptions {
  headers: HeaderMap,
  timeout: Option<Duration>,
  proxy: Option<Proxy>,
}

impl HttpClientOptions {
  pub fn new() -> HttpClientOptions {
    HttpClientOptions {
      headers: HeaderMap::new(),
      timeout: None,
      proxy: None,
    }
  }

  pub fn set_headers(&mut self, headers: HeaderMap) {
    self.headers.extend(headers);
  }

  pub fn set_header(&mut self, key: HeaderName, value: HeaderValue) {
    self.headers.insert(key, value);
  }

  pub fn set_proxy(&mut self, proxy: Proxy) {
    self.proxy = Some(proxy);
  }

  pub fn set_timeout(&mut self, timeout: Duration) {
    self.timeout = Some(timeout);
  }

  pub fn contains_header(&self, key: impl AsHeaderName) -> bool {
    self.headers.contains_key(key)
  }

  pub fn build_client(self) -> Result<Client> {
    let mut client_builder = Client::builder();
    if let Some(timeout) = self.timeout {
      client_builder = client_builder.timeout(timeout);
    }
    if let Some(proxy) = self.proxy {
      client_builder = client_builder.proxy(proxy);
    }
    client_builder
      .default_headers(self.headers)
      .build()
      .context(ReqwestClientSnafu)
  }
}

pub struct HttpClient;

impl HttpClient {
  pub async fn post<T: Serialize + ?Sized, U: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    json: &T,
  ) -> Result<U> {
    let res = client
      .post(url)
      .json(json)
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<U>()
      .await
      .context(ReqwestClientSnafu)?;
    Ok(res)
  }
}
