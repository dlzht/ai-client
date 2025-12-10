use std::time::Duration;

use reqwest::{
  Client, Method, Proxy,
  header::{AUTHORIZATION, AsHeaderName, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, ReqwestHeaderSnafu, Result};

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

  pub fn with_headers(mut self, headers: HeaderMap) -> Self {
    self.headers.extend(headers);
    self
  }

  pub fn with_header(mut self, key: HeaderName, value: HeaderValue) -> Self {
    self.headers.insert(key, value);
    self
  }

  pub fn with_proxy(mut self, proxy: Proxy) -> Self {
    self.proxy = Some(proxy);
    self
  }

  pub fn with_timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
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

pub struct HttpComponent;

impl HttpComponent {
  pub fn new_client_with_api_key(api_key: impl AsRef<str>) -> Result<Client> {
    let authorization = HeaderValue::from_str(api_key.as_ref()).map_err(|_| {
      ReqwestHeaderSnafu {
        header: api_key.as_ref().to_string(),
      }
      .build()
    })?;
    let options = HttpClientOptions::new()
      .with_header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
      .with_header(AUTHORIZATION, authorization);
    options.build_client()
  }

  pub fn new_client_with_options(
    api_key: impl AsRef<str>,
    mut options: HttpClientOptions,
  ) -> Result<Client> {
    if !options.contains_header(CONTENT_TYPE) {
      options = options.with_header(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    }
    if !options.contains_header(AUTHORIZATION) {
      let authorization = HeaderValue::from_str(api_key.as_ref()).map_err(|_| {
        ReqwestHeaderSnafu {
          header: api_key.as_ref().to_string(),
        }
        .build()
      })?;
      options = options.with_header(AUTHORIZATION, authorization);
    }
    options.build_client()
  }
  pub fn static_header(key: &'static str, value: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
      HeaderName::from_static(key),
      HeaderValue::from_static(value),
    );
    headers
  }

  pub async fn get<Q: Serialize + ?Sized, B: Serialize + ?Sized, R: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    headers: Option<HeaderMap>,
    queries: Option<&Q>,
    json: Option<&B>,
  ) -> Result<R> {
    Self::execute(client, Method::GET, url, headers, queries, json).await
  }
  pub async fn post<Q: Serialize + ?Sized, B: Serialize + ?Sized, R: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    headers: Option<HeaderMap>,
    queries: Option<&Q>,
    json: Option<&B>,
  ) -> Result<R> {
    Self::execute(client, Method::POST, url, headers, queries, json).await
  }

  async fn execute<Q: Serialize + ?Sized, B: Serialize + ?Sized, R: for<'de> Deserialize<'de>>(
    client: &Client,
    method: Method,
    url: &str,
    headers: Option<HeaderMap>,
    queries: Option<&Q>,
    json: Option<&B>,
  ) -> Result<R> {
    let mut req = client.request(method, url);
    if let Some(headers) = headers {
      req = req.headers(headers);
    }
    if let Some(queries) = queries {
      req = req.query(queries);
    }
    if let Some(json) = json {
      req = req.json(json);
    }
    let res = req
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<R>()
      .await
      .context(ReqwestClientSnafu)?;
    Ok(res)
  }
}
