use ai_client_common::common::MessageRole;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskRes {
  #[serde(rename = "output")]
  pub output: SyncTaskOutput,

  #[serde(rename = "usage")]
  pub usage: TokenUsageRes,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskOutput {
  #[serde(rename = "choices")]
  pub choices: Vec<SyncTaskChoice>,

  #[serde(rename = "task_metric")]
  pub metric: SyncTaskMetric,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskChoice {
  #[serde(rename = "finish_reason")]
  pub finish_reason: Option<String>,

  #[serde(rename = "message")]
  pub message: SyncTaskMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskMessage {
  #[serde(rename = "role")]
  pub role: Option<MessageRole>,

  #[serde(rename = "content")]
  pub content: Vec<SyncTaskContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskContent {
  #[serde(rename = "image")]
  pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncTaskMetric {
  #[serde(rename = "TOTAL")]
  pub total: i32,

  #[serde(rename = "SUCCEEDED")]
  pub succeeded: i32,

  #[serde(rename = "FAILED")]
  pub failed: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenUsageRes {
  #[serde(rename = "image_count")]
  pub image_count: i32,

  #[serde(rename = "width")]
  pub width: i32,

  #[serde(rename = "height")]
  pub height: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsyncTaskRes {
  #[serde(rename = "output")]
  pub output: AsyncTaskOutput,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsyncTaskOutput {
  #[serde(rename = "task_id")]
  pub task_id: String,

  #[serde(rename = "task_status")]
  pub task_status: AsyncTaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsyncResultRes {
  #[serde(rename = "output")]
  pub output: AsyncResultOutput,

  #[serde(rename = "usage")]
  pub usage: TokenUsageRes,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsyncResultOutput {
  #[serde(rename = "task_id")]
  pub task_id: String,

  #[serde(rename = "task_status")]
  pub task_status: AsyncTaskStatus,

  #[serde(rename = "submit_time")]
  pub submit_time: String,

  #[serde(rename = "scheduled_time")]
  pub scheduled_time: String,

  #[serde(rename = "end_time")]
  pub end_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsyncResultData {
  #[serde(rename = "orig_prompt")]
  pub orig_prompt: String,

  #[serde(rename = "actual_prompt")]
  pub actual_prompt: String,

  #[serde(rename = "url")]
  pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AsyncTaskStatus {
  #[serde(rename = "PENDING")]
  Pending,

  #[serde(rename = "RUNNING")]
  Running,

  #[serde(rename = "SUCCEEDED")]
  Succeeded,

  #[serde(rename = "FAILED")]
  Failed,

  #[serde(rename = "CANCELED")]
  Canceled,

  #[serde(rename = "UNKNOWN")]
  Unknown,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum QianWenImageRes<T> {
  Failure(QianWenImageErr),
  Success(T),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenImageErr {
  #[serde(rename = "code")]
  pub code: String,

  #[serde(rename = "message")]
  pub message: String,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[cfg(test)]
mod test {
  use crate::image::response::{QianWenImageRes, SyncTaskOutput, SyncTaskRes};

  #[test]
  fn test_deserialize_res() {
    let json1 = "{\"output\":{\"choices\":[{\"finish_reason\":\"stop\",\"message\":{\"role\":\"assistant\",\"content\":[{\"image\":\"https://dashscope-result-sz.oss-cn-shenzhen.aliyuncs.com/xxx.png?Expires=xxxx\"}]}}],\"task_metric\":{\"TOTAL\":1,\"FAILED\":0,\"SUCCEEDED\":1}},\"usage\":{\"width\":1328,\"image_count\":1,\"height\":1328},\"request_id\":\"7a270c86-db58-9faf-b403-xxxxxx\"}";
    assert!(serde_json::from_str::<QianWenImageRes<SyncTaskOutput>>(json1).is_ok());

    let json2 = "{\"code\":\"InvalidApiKey\",\"message\":\"Invalid API-key provided.\",\"request_id\":\"745f0c85-ea30-97a5-b067-34824920af37\"}";
    assert!(serde_json::from_str::<QianWenImageRes<SyncTaskOutput>>(json2).is_ok());
  }
}
