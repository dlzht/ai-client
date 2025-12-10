use ai_client_common::{client::HttpComponent, common::NoneType, errors::Result};
use reqwest::Client;

use crate::{model::GeminiModel, request::GeminiReq, response::GeminiRes};

pub const DEFAULT_GENERATE_CONTENT_URL: &'static str =
  " https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent";
pub const DEFAULT_STREAM_GENERATE_CONTENT_URL: &'static str =
  "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent";

pub struct GeminiEndpoints {
  pub(crate) urls: GeminiUrl,
  pub(crate) model: GeminiModel,
}

impl GeminiEndpoints {
  pub fn new() -> Self {
    GeminiEndpoints {
      urls: GeminiUrl {
        generate_content: DEFAULT_GENERATE_CONTENT_URL.to_string(),
        stream_generate_content: DEFAULT_STREAM_GENERATE_CONTENT_URL.to_string(),
      },
      model: GeminiModel::Gemini3ProPreview,
    }
  }

  pub fn with_generate_content_url(mut self, url: impl Into<String>) -> Self {
    self.urls.generate_content = url.into();
    self
  }

  pub fn with_stream_generate_content_url(mut self, url: impl Into<String>) -> Self {
    self.urls.stream_generate_content = url.into();
    self
  }

  pub fn with_model(mut self, model: GeminiModel) -> Self {
    self.model = model;
    self
  }

  pub async fn generate_content(&self, client: &Client, request: &GeminiReq) -> Result<GeminiRes> {
    let model = request.model.as_ref().unwrap_or(&self.model);
    let url = self.urls.generate_content.replace("{}", model.to_str());
    HttpComponent::post::<NoneType, GeminiReq, GeminiRes>(client, &url, None, None, Some(request))
      .await
  }

  pub async fn stream_generate_content(
    &self,
    client: &Client,
    request: &GeminiReq,
  ) -> Result<GeminiRes> {
    let model = request.model.as_ref().unwrap_or(&self.model);
    let url = self.urls.generate_content.replace("{}", model.to_str());
    HttpComponent::post::<NoneType, GeminiReq, GeminiRes>(client, &url, None, None, Some(request))
      .await
  }
}

pub struct GeminiUrl {
  pub(crate) generate_content: String,
  pub(crate) stream_generate_content: String,
}
