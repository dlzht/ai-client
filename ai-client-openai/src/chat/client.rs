use ai_client_common::{
  client::HttpClientOptions,
  constant::DEFAULT_HEADER_USER_AGENT,
  errors::{PlainMessageSnafu, ReqwestClientSnafu, Result},
};
use reqwest::{
  header::{HeaderValue, AUTHORIZATION, USER_AGENT},
  Client,
};
use snafu::ResultExt;

use crate::chat::{
  constant::OPEN_AI_DEFAULT_CHAT_ENDPOINT, request::OpenAiChatReq, response::OpenAiChatRes,
};

pub struct OpenAiChatClient {
  // api_key: String,
  chat_endpoint: String,
  http_client: Client,
}

impl OpenAiChatClient {
  pub async fn chat_completion(&self, request: &OpenAiChatReq) -> Result<OpenAiChatRes> {
    let res = self
      .http_client
      .post(&self.chat_endpoint)
      .json(&request)
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<OpenAiChatRes>()
      .await
      .context(ReqwestClientSnafu)?;
    Ok(res)
  }
}

pub struct OpenAiChatClientBuilder {
  api_key: String,
  chat_endpoint: Option<String>,
  http_options: HttpClientOptions,
}

impl OpenAiChatClientBuilder {
  pub fn new(api_key: impl Into<String>) -> Self {
    OpenAiChatClientBuilder {
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

  pub fn build(mut self) -> Result<OpenAiChatClient> {
    let _ = self.process_default_options()?;
    let http_client = self.http_options.build_client()?;
    let client = OpenAiChatClient {
      // api_key: self.api_key,
      chat_endpoint: self
        .chat_endpoint
        .unwrap_or(OPEN_AI_DEFAULT_CHAT_ENDPOINT.to_string()),
      http_client,
    };
    Ok(client)
  }
}

impl OpenAiChatClientBuilder {
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
