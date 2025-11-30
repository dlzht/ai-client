use ai_client_common::{client::HttpComponent, common::DummyType, errors::Result};
use reqwest::Client;

use crate::chat::{
  model::QianWenChatModel,
  request::QianWenChatReq,
  response::{ChatCompletionRes, QianWenChatRes},
};

pub const DEFAULT_CHAT_COMPLETION_URL: &'static str =
  "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

pub struct QianWenChatEndpoint {
  pub(crate) urls: QianWenChatUrl,
  pub(crate) model: QianWenChatModel,
}

impl QianWenChatEndpoint {
  pub fn new() -> Self {
    QianWenChatEndpoint {
      urls: QianWenChatUrl {
        chat_completion: DEFAULT_CHAT_COMPLETION_URL.to_string(),
      },
      model: QianWenChatModel::Qwen3Max,
    }
  }

  pub fn with_model(mut self, model: impl Into<QianWenChatModel>) -> Self {
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
    request: &QianWenChatReq,
  ) -> Result<QianWenChatRes<ChatCompletionRes>> {
    HttpComponent::post::<DummyType, QianWenChatReq, QianWenChatRes<ChatCompletionRes>>(
      client,
      &self.urls.chat_completion,
      None,
      None,
      Some(request),
    )
    .await
  }
}

pub struct QianWenChatUrl {
  chat_completion: String,
}
