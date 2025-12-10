use ai_client_common::{client::HttpComponent, common::NoneType, errors::Result};
use reqwest::Client;

use crate::chat::{model::OpenAiChatModel, request::OpenAiChatReq, response::OpenAiChatRes};

pub const DEFAULT_CHAT_COMPLETION_URL: &'static str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAiChatEndpoint {
  pub(crate) urls: OpenAiUrl,
  pub(crate) model: OpenAiChatModel,
}

pub struct OpenAiUrl {
  pub(crate) chat_completion: String,
}

impl OpenAiChatEndpoint {
  pub fn new() -> Self {
    OpenAiChatEndpoint {
      urls: OpenAiUrl {
        chat_completion: DEFAULT_CHAT_COMPLETION_URL.to_string(),
      },
      model: OpenAiChatModel::Gpt4,
    }
  }

  pub fn with_model(mut self, model: impl Into<OpenAiChatModel>) -> Self {
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
    request: &OpenAiChatReq,
  ) -> Result<OpenAiChatRes> {
    HttpComponent::post::<NoneType, OpenAiChatReq, OpenAiChatRes>(
      client,
      &self.urls.chat_completion,
      None,
      None,
      Some(request),
    )
    .await
  }
}
