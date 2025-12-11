use ai_client_common::{
  client::{HttpClientOptions, HttpComponent},
  errors::Result,
};
use reqwest::Client;

use crate::{endpoint::DeepseekChatEndpoint, model::DeepSeekChatModel, request::DeepSeekChatReq, response::{DeepSeekChatRes, DeepSeekRes}};

pub struct DeepSeekClient {
  http_client: Client,
  api_endpoints: DeepSeekEndpoints,
}

impl DeepSeekClient {
  pub fn new(api_key: impl AsRef<str>) -> Result<Self> {
    let client = DeepSeekClient {
      http_client: HttpComponent::new_client_with_api_key(api_key)?,
      api_endpoints: DeepSeekEndpoints::new(),
    };
    Ok(client)
  }

  pub fn new_with_options(api_key: impl AsRef<str>, options: HttpClientOptions) -> Result<Self> {
    let client = DeepSeekClient {
      http_client: HttpComponent::new_client_with_options(api_key, options)?,
      api_endpoints: DeepSeekEndpoints::new(),
    };
    Ok(client)
  }

  pub fn with_chat_model(mut self, model: impl Into<DeepSeekChatModel>) -> Self {
    self.api_endpoints.chat_completion = self.api_endpoints.chat_completion.with_model(model);
    self
  }

  pub fn with_chat_completion_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.chat_completion = self
      .api_endpoints
      .chat_completion
      .with_chat_completion_url(url);
    self
  }
  
  pub async fn chat_completion(&self, req: &DeepSeekChatReq) -> Result<DeepSeekRes<DeepSeekChatRes>> {
    self.api_endpoints.chat_completion.chat_completion(&self.http_client, req).await
  }
}

pub struct DeepSeekEndpoints {
  pub(crate) chat_completion: DeepseekChatEndpoint,
}

impl DeepSeekEndpoints {
  pub fn new() -> Self {
    DeepSeekEndpoints {
      chat_completion: DeepseekChatEndpoint::new(),
    }
  }
}
