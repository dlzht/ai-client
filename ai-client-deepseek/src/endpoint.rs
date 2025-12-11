use ai_client_common::{client::HttpComponent, common::NoneType, errors::Result};
use reqwest::Client;

use crate::{
  model::DeepSeekChatModel,
  request::DeepSeekChatReq,
  response::{DeepSeekChatRes, DeepSeekRes},
};

pub const DEFAULT_CHAT_COMPLETION_URL: &'static str = "https://api.deepseek.com/chat/completions";

pub struct DeepseekChatEndpoint {
  pub(crate) urls: DeepseekChatUrl,
  pub(crate) model: DeepSeekChatModel,
}

impl DeepseekChatEndpoint {
  pub fn new() -> Self {
    DeepseekChatEndpoint {
      urls: DeepseekChatUrl {
        chat_completion: DEFAULT_CHAT_COMPLETION_URL.to_string(),
      },
      model: DeepSeekChatModel::DeepSeekV32,
    }
  }

  pub fn with_model(mut self, model: impl Into<DeepSeekChatModel>) -> Self {
    self.model = model.into();
    self
  }

  pub fn with_chat_completion_url(mut self, url: impl Into<String>) -> Self {
    self.urls.chat_completion = url.into();
    self
  }

  pub async fn chat_completion(
    &self,
    client: &Client,
    request: &DeepSeekChatReq,
  ) -> Result<DeepSeekRes<DeepSeekChatRes>> {
    HttpComponent::post::<NoneType, DeepSeekChatReq, DeepSeekRes<DeepSeekChatRes>>(
      client,
      &self.urls.chat_completion,
      None,
      None,
      Some(request),
    )
    .await
  }
}

pub struct DeepseekChatUrl {
  pub(crate) chat_completion: String,
}