use ai_client_common::{client::HttpComponent, common::NoneType, errors::Result};
use reqwest::Client;

use crate::image::{
  model::QianWenImageModel,
  request::QianWenImageReq,
  response::{AsyncResultRes, AsyncTaskRes, QianWenImageRes, SyncTaskRes},
};

pub const DEFAULT_TEXT_TO_IMAGE_SYNC_TASK_URL: &'static str =
  "https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation";
pub const DEFAULT_TEXT_TO_IMAGE_ASYNC_TASK_URL: &'static str =
  "https://dashscope.aliyuncs.com/api/v1/services/aigc/text2image/image-synthesis";
pub const DEFAULT_TEXT_TO_IMAGE_ASYNC_RESULT_URL: &'static str =
  "https://dashscope.aliyuncs.com/api/v1/tasks/{}";

pub struct QianWenImageEndpoint {
  urls: QianWenImageUrl,
  model: QianWenImageModel,
}

impl QianWenImageEndpoint {
  pub fn new() -> Self {
    QianWenImageEndpoint {
      urls: QianWenImageUrl {
        text_to_image_sync_task: DEFAULT_TEXT_TO_IMAGE_SYNC_TASK_URL.to_string(),
        text_to_image_async_task: DEFAULT_TEXT_TO_IMAGE_ASYNC_TASK_URL.to_string(),
        text_to_image_async_result: DEFAULT_TEXT_TO_IMAGE_ASYNC_RESULT_URL.to_string(),
      },
      model: QianWenImageModel::QwenImagePlus,
    }
  }

  pub fn with_model(mut self, model: impl Into<QianWenImageModel>) -> Self {
    self.model = model.into();
    self
  }

  pub fn with_text_to_image_sync_task_url(mut self, url: impl Into<String>) -> Self {
    self.urls.text_to_image_sync_task = url.into();
    self
  }

  pub fn with_text_to_image_async_task_url(mut self, url: impl Into<String>) -> Self {
    self.urls.text_to_image_async_task = url.into();
    self
  }

  pub fn with_text_to_image_async_result_url(mut self, url: impl Into<String>) -> Self {
    self.urls.text_to_image_async_result = url.into();
    self
  }

  pub async fn text_to_image_sync_task(
    &self,
    client: &Client,
    request: &QianWenImageReq,
  ) -> Result<QianWenImageRes<SyncTaskRes>> {
    HttpComponent::post::<NoneType, QianWenImageReq, QianWenImageRes<SyncTaskRes>>(
      client,
      &self.urls.text_to_image_sync_task,
      None,
      None,
      Some(request),
    )
    .await
  }

  pub async fn text_to_image_async_task(
    &self,
    client: &Client,
    request: &QianWenImageReq,
  ) -> Result<QianWenImageRes<AsyncTaskRes>> {
    let header = HttpComponent::static_header("X-DashScope-Async", "enable");
    HttpComponent::post::<NoneType, QianWenImageReq, QianWenImageRes<AsyncTaskRes>>(
      client,
      &self.urls.text_to_image_async_task,
      Some(header),
      None,
      Some(request),
    )
    .await
  }

  pub async fn text_to_image_async_result(
    &self,
    client: &Client,
    task_id: &str,
  ) -> Result<QianWenImageRes<AsyncResultRes>> {
    let url = self
      .urls
      .text_to_image_async_result
      .replace("{task_id}", task_id);
    HttpComponent::get::<NoneType, NoneType, QianWenImageRes<AsyncResultRes>>(
      client, &url, None, None, None,
    )
    .await
  }
}

pub struct QianWenImageUrl {
  text_to_image_sync_task: String,
  text_to_image_async_task: String,
  text_to_image_async_result: String,
}
