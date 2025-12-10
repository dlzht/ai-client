use ai_client_common::{
  client::{HttpClientOptions, HttpComponent},
  errors::Result,
};
use reqwest::Client;

use crate::{
  endpoint::GeminiEndpoints, model::GeminiModel, request::GeminiReq, response::GeminiRes,
};

pub struct GeminiClient {
  http_client: Client,
  api_endpoints: GeminiEndpoints,
}

impl GeminiClient {
  pub fn new(api_key: impl AsRef<str>) -> Result<Self> {
    let client = GeminiClient {
      http_client: HttpComponent::new_client_with_api_key(api_key)?,
      api_endpoints: GeminiEndpoints::new(),
    };
    Ok(client)
  }

  pub fn new_with_options(api_key: impl AsRef<str>, options: HttpClientOptions) -> Result<Self> {
    let client = GeminiClient {
      http_client: HttpComponent::new_client_with_options(api_key, options)?,
      api_endpoints: GeminiEndpoints::new(),
    };
    Ok(client)
  }

  pub fn with_generate_content_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.urls.generate_content = url.into();
    self
  }

  pub fn with_stream_generate_content_url(mut self, url: impl Into<String>) -> Self {
    self.api_endpoints.urls.stream_generate_content = url.into();
    self
  }

  pub fn with_model(mut self, model: impl Into<GeminiModel>) -> Self {
    self.api_endpoints.model = model.into();
    self
  }

  pub async fn generate_content(&self, request: &GeminiReq) -> Result<GeminiRes> {
    self
      .api_endpoints
      .generate_content(&self.http_client, request)
      .await
  }

  pub async fn stream_generate_content(&self, request: &GeminiReq) -> Result<GeminiRes> {
    self
      .api_endpoints
      .generate_content(&self.http_client, request)
      .await
  }
}
