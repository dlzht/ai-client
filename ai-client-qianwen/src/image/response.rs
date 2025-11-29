use serde::{Deserialize, Serialize};
use ai_client_common::common::MessageRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenImageData {
  #[serde(rename = "output")]
  pub output: OutputData,

  #[serde(rename = "usage")]
  pub usage: UsageData,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputData {
  #[serde(rename = "choices")]
  pub choices: Vec<OutputChoice>,

  #[serde(rename = "task_metric")]
  pub metric: OutputMetric,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputChoice {
  #[serde(rename = "finish_reason")]
  pub finish_reason: Option<String>,

  #[serde(rename = "message")]
  pub message: OutputMessage,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMessage {
  #[serde(rename = "role")]
  pub role: Option<MessageRole>,

  #[serde(rename = "content")]
  pub content: Vec<OutputContent>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputContent {
  #[serde(rename = "image")]
  pub image: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMetric {
  #[serde(rename = "TOTAL")]
  pub total: i32,

  #[serde(rename = "SUCCEEDED")]
  pub succeeded: i32,

  #[serde(rename = "FAILED")]
  pub failed: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageData {
  #[serde(rename = "image_count")]
  pub image_count: i32,

  #[serde(rename = "width")]
  pub width: i32,

  #[serde(rename = "height")]
  pub height: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum QianWenImageRes {
  Failure(QianWenImageErr),
  Success(QianWenImageData)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenImageErr {

  #[serde(rename = "code")]
  code: String,

  #[serde(rename = "message")]
  message: String,

  #[serde(rename = "request_id")]
  request_id: String
}

#[cfg(test)]
mod test {
  use crate::image::response::{QianWenImageData, QianWenImageRes};

  #[test]
  fn test_deserialize_res() {
    let json1 = "{\"output\":{\"choices\":[{\"finish_reason\":\"stop\",\"message\":{\"role\":\"assistant\",\"content\":[{\"image\":\"https://dashscope-result-sz.oss-cn-shenzhen.aliyuncs.com/xxx.png?Expires=xxxx\"}]}}],\"task_metric\":{\"TOTAL\":1,\"FAILED\":0,\"SUCCEEDED\":1}},\"usage\":{\"width\":1328,\"image_count\":1,\"height\":1328},\"request_id\":\"7a270c86-db58-9faf-b403-xxxxxx\"}";
    assert!(serde_json::from_str::<QianWenImageRes>(json1).is_ok());

    let json2 = "{\"code\":\"InvalidApiKey\",\"message\":\"Invalid API-key provided.\",\"request_id\":\"745f0c85-ea30-97a5-b067-34824920af37\"}";
    assert!(serde_json::from_str::<QianWenImageRes>(json2).is_ok());
  }
}