use ai_client_common::{
  client::{HttpClient, HttpClientOptions},
  constant::DEFAULT_HEADER_USER_AGENT,
  errors::{PlainMessageSnafu, Result},
};
use reqwest::{
  header::{HeaderValue, AUTHORIZATION, USER_AGENT},
  Client,
};

use crate::chat::{
  request::QianWenChatReq,
  response::{QianWenChatData, },
};
use crate::chat::response::QianWenChatRes;
use crate::constant::QIAN_WEN_DEFAULT_CHAT_ENDPOINT;

pub struct QianWenChatClient {
  // api_key: String,
  chat_endpoint: String,
  http_client: Client,
}

impl QianWenChatClient {
  pub async fn chat_completion(
    &self,
    request: &QianWenChatReq,
  ) -> Result<QianWenChatRes<QianWenChatData>> {
    HttpClient::post::<QianWenChatReq, QianWenChatRes<QianWenChatData>>(
      &self.http_client,
      &self.chat_endpoint,
      request,
    )
    .await
  }
}

pub struct QianWenChatClientBuilder {
  api_key: String,
  chat_endpoint: Option<String>,
  http_options: HttpClientOptions,
}

impl QianWenChatClientBuilder {
  pub fn new(api_key: impl Into<String>) -> Self {
    QianWenChatClientBuilder {
      api_key: api_key.into(),
      chat_endpoint: None,
      http_options: HttpClientOptions::new(),
    }
  }

  pub fn chat_endpoint(mut self, path: impl Into<String>) -> Self {
    self.chat_endpoint = Some(path.into());
    self
  }

  pub fn http_options(mut self, options: HttpClientOptions) -> Self {
    self.http_options = options;
    self
  }

  pub fn build(mut self) -> Result<QianWenChatClient> {
    let _ = self.process_default_options()?;
    let http_client = self.http_options.build_client()?;
    let client = QianWenChatClient {
      // api_key: self.api_key,
      chat_endpoint: self
        .chat_endpoint
        .unwrap_or(QIAN_WEN_DEFAULT_CHAT_ENDPOINT.to_string()),
      http_client,
    };
    Ok(client)
  }
}

impl QianWenChatClientBuilder {
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
