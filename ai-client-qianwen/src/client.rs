use ai_client_common::{
  client::HttpClientOptions,
  errors::{ReqwestHeaderSnafu, Result},
};
use reqwest::{
  Client,
  header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue},
};

use crate::{
  chat::{
    endpoint::QianWenChatEndpoint,
    model::QianWenChatModel,
    request::{MessageParam, QianWenChatReq},
    response::{ChatCompletionRes, QianWenChatRes},
  },
  image::{
    endpoint::QianWenImageEndpoint,
    model::QianWenImageModel,
    request::QianWenImageReq,
    response::{AsyncResultRes, AsyncTaskRes, QianWenImageRes, SyncTaskRes},
  },
};

pub struct QianWenClient {
  http_client: Client,
  api_endpoints: QianWenEndpoints,
}

impl QianWenClient {
  pub fn new(api_key: impl AsRef<str>) -> Result<Self> {
    let authorization = HeaderValue::from_str(api_key.as_ref()).map_err(|_| {
      ReqwestHeaderSnafu {
        header: api_key.as_ref().to_string(),
      }
      .build()
    })?;
    let options = HttpClientOptions::new()
      .with_header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
      .with_header(AUTHORIZATION, authorization);
    let client = QianWenClient {
      http_client: options.build_client()?,
      api_endpoints: QianWenEndpoints::new(),
    };
    Ok(client)
  }

  pub fn new_with_options(
    api_key: impl AsRef<str>,
    mut options: HttpClientOptions,
  ) -> Result<Self> {
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
    let client = QianWenClient {
      http_client: options.build_client()?,
      api_endpoints: QianWenEndpoints::new(),
    };
    Ok(client)
  }

  pub fn with_chat_model(mut self, model: impl Into<QianWenChatModel>) -> Self {
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

  pub fn with_image_model(mut self, model: impl Into<QianWenImageModel>) -> Self {
    self.api_endpoints.image_generation = self.api_endpoints.image_generation.with_model(model);
    self
  }

  pub fn with_text_to_image_sync_task_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.image_generation = self
      .api_endpoints
      .image_generation
      .with_text_to_image_sync_task_url(url);
    self
  }

  pub fn with_text_to_image_async_task_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.image_generation = self
      .api_endpoints
      .image_generation
      .with_text_to_image_async_task_url(url);
    self
  }

  pub fn with_image_to_image_async_result_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.image_generation = self
      .api_endpoints
      .image_generation
      .with_text_to_image_async_result_url(url);
    self
  }
}

impl QianWenClient {
  pub fn req_chat_completion_with_messages(&self, messages: Vec<MessageParam>) -> QianWenChatReq {
    QianWenChatReq::new(self.api_endpoints.chat_completion.model.clone(), messages)
  }

  pub fn req_chat_completion_with_user_text(&self, text: impl Into<String>) -> QianWenChatReq {
    QianWenChatReq::new(
      self.api_endpoints.chat_completion.model.clone(),
      vec![MessageParam::new_user_message_with_text(text)],
    )
  }
}

impl QianWenClient {
  pub async fn chat_completion(
    &self,
    request: &QianWenChatReq,
  ) -> Result<QianWenChatRes<ChatCompletionRes>> {
    self
      .api_endpoints
      .chat_completion
      .chat_completion(&self.http_client, request)
      .await
  }

  pub async fn text_to_image_sync_task(
    &self,
    req: &QianWenImageReq,
  ) -> Result<QianWenImageRes<SyncTaskRes>> {
    self
      .api_endpoints
      .image_generation
      .text_to_image_sync_task(&self.http_client, req)
      .await
  }

  pub async fn text_to_image_async_task(
    &self,
    req: &QianWenImageReq,
  ) -> Result<QianWenImageRes<AsyncTaskRes>> {
    self
      .api_endpoints
      .image_generation
      .text_to_image_async_task(&self.http_client, req)
      .await
  }

  pub async fn text_to_image_async_result(
    &self,
    task_id: &str,
  ) -> Result<QianWenImageRes<AsyncResultRes>> {
    self
      .api_endpoints
      .image_generation
      .text_to_image_async_result(&self.http_client, task_id.as_ref())
      .await
  }
}

struct QianWenEndpoints {
  chat_completion: QianWenChatEndpoint,
  image_generation: QianWenImageEndpoint,
}

impl QianWenEndpoints {
  fn new() -> Self {
    QianWenEndpoints {
      chat_completion: QianWenChatEndpoint::new(),
      image_generation: QianWenImageEndpoint::new(),
    }
  }
}
