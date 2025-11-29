use ai_client_common::{
  client::{HttpClient, HttpClientOptions},
  constant::DEFAULT_HEADER_USER_AGENT,
  errors::{PlainMessageSnafu, Result},
};
use reqwest::{
  header::{HeaderValue, AUTHORIZATION, USER_AGENT},
  Client,
};
use crate::constant::QIAN_WEN_DEFAULT_IMAGE_ENDPOINT;
use crate::image::request::QianWenImageReq;
use crate::image::response::QianWenImageRes;

pub struct QianWenImageClient {
  // api_key: String,
  chat_endpoint: String,
  http_client: Client,
}

impl QianWenImageClient {
  pub async fn generate_image(
    &self,
    request: &QianWenImageReq,
  ) -> Result<QianWenImageRes> {
    HttpClient::post::<QianWenImageReq, QianWenImageRes>(
      &self.http_client,
      &self.chat_endpoint,
      request,
    )
    .await
  }
}

pub struct QianWenImageClientBuilder {
  api_key: String,
  chat_endpoint: Option<String>,
  http_options: HttpClientOptions,
}

impl QianWenImageClientBuilder {
  pub fn new(api_key: impl Into<String>) -> Self {
    QianWenImageClientBuilder {
      api_key: api_key.into(),
      chat_endpoint: None,
      http_options: HttpClientOptions::new(),
    }
  }

  pub fn image_endpoint(mut self, path: impl Into<String>) -> Self {
    self.chat_endpoint = Some(path.into());
    self
  }

  pub fn http_options(mut self, options: HttpClientOptions) -> Self {
    self.http_options = options;
    self
  }

  pub fn build(mut self) -> Result<QianWenImageClient> {
    let _ = self.process_default_options()?;
    let http_client = self.http_options.build_client()?;
    let client = QianWenImageClient {
      // api_key: self.api_key,
      chat_endpoint: self
        .chat_endpoint
        .unwrap_or(QIAN_WEN_DEFAULT_IMAGE_ENDPOINT.to_string()),
      http_client,
    };
    Ok(client)
  }
}

impl QianWenImageClientBuilder {
  fn process_default_options(&mut self) -> Result<()> {
    if !self.http_options.contains_header(USER_AGENT) {
      self
        .http_options
        .set_header(USER_AGENT, DEFAULT_HEADER_USER_AGENT);
    }
    if !self.http_options.contains_header(AUTHORIZATION) {
      let authorization =
        HeaderValue::from_str(&format!("Bearer {}", self.api_key)).map_err(|_| {
          PlainMessageSnafu {
            message: format!("Invalid header value: {}", self.api_key),
          }
          .build()
        })?;
      self.http_options.set_header(AUTHORIZATION, authorization);
    }
    Ok(())
  }
}
