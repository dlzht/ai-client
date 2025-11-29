use serde::{Deserialize, Serialize};
use ai_client_common::common::MessageRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChatRes {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "object")]
  pub object: String,

  #[serde(rename = "created")]
  pub created: i32,

  #[serde(rename = "model")]
  pub model: String,

  #[serde(rename = "choices")]
  pub choices: Vec<ChoiceData>,

  #[serde(rename = "usage")]
  pub usage: Option<UsageData>,

  #[serde(rename = "service_tier")]
  pub service_tier: Option<String>,
  // #[serde(rename = "system_fingerprint")]
  // system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceData {
  #[serde(rename = "index")]
  pub index: i32,

  #[serde(rename = "finish_reason")]
  pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceMessage {
  #[serde(rename = "role")]
  pub role: MessageRole,

  #[serde(rename = "content")]
  pub content: String,

  #[serde(rename = "annotation")]
  pub refusal: Option<String>,

  #[serde(rename = "annotations")]
  pub annotations: Vec<ChoiceAnnotation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceAnnotation {
  #[serde(rename = "type")]
  pub kind: String,

  #[serde(rename = "url_citation")]
  pub url: ChoiceAnnotationUrl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceAnnotationUrl {
  #[serde(rename = "start_index")]
  pub start_index: i32,

  #[serde(rename = "end_index")]
  pub end_index: i32,

  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "url")]
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageData {
  #[serde(rename = "prompt_tokens")]
  pub prompt_tokens: i32,

  #[serde(rename = "completion_tokens")]
  pub completion_tokens: i32,

  #[serde(rename = "total_tokens")]
  pub total_tokens: i32,

  #[serde(rename = "completion_tokens_details")]
  pub completion_tokens_details: Option<CompletionTokensDetails>,

  #[serde(rename = "prompt_tokens_details")]
  pub prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetails {
  #[serde(rename = "accepted_prediction_tokens")]
  pub accepted_prediction_tokens: Option<i32>,

  #[serde(rename = "audio_tokens")]
  pub audio_tokens: Option<i32>,

  #[serde(rename = "reasoning_tokens")]
  pub reasoning_tokens: Option<i32>,

  #[serde(rename = "rejected_prediction_tokens")]
  pub rejected_prediction_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetails {
  #[serde(rename = "text_tokens")]
  pub audio_tokens: Option<i32>,

  #[serde(rename = "cached_tokens")]
  pub cached_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChatChunkRes {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "object")]
  pub object: String,

  #[serde(rename = "created")]
  pub created: i32,

  #[serde(rename = "model")]
  pub model: String,

  #[serde(rename = "choices")]
  pub choices: Option<Vec<ChunkChoiceData>>,

  #[serde(rename = "usage")]
  pub usage: Option<UsageData>,

  #[serde(rename = "service_tier")]
  pub service_tier: Option<String>,
  // #[serde(rename = "system_fingerprint")]
  // system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceData {
  #[serde(rename = "index")]
  pub index: i32,

  #[serde(rename = "finish_reason")]
  pub finish_reason: Option<String>,

  #[serde(rename = "delta")]
  pub delta: ChunkChoiceDelta,

  #[serde(rename = "logprobs")]
  pub log_prob: Option<ChunkChoiceLogProb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceDelta {
  #[serde(rename = "content")]
  pub content: String,

  #[serde(rename = "refusal")]
  pub refusal: Option<String>,

  #[serde(rename = "role")]
  pub role: MessageRole,

  #[serde(rename = "tool_calls", default)]
  pub tool_calls: Option<Vec<ChunkChoiceToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceToolCall {
  #[serde(rename = "index")]
  pub index: i32,

  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "type")]
  pub kind: String,

  #[serde(rename = "function")]
  pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
  #[serde(rename = "arguments")]
  pub arguments: String,

  #[serde(rename = "name")]
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceLogProb {
  #[serde(rename = "content")]
  content: Option<Vec<ChunkChoiceLogProbData>>,

  #[serde(rename = "refusal")]
  refusal: Option<Vec<ChunkChoiceLogProbData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceLogProbData {
  #[serde(rename = "bytes")]
  pub bytes: Option<Vec<u8>>,

  #[serde(rename = "logprob")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,

  #[serde(rename = "top_logprobs")]
  pub top_log_prob: Option<Vec<ChunkChoiceTopLogProb>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceTopLogProb {
  #[serde(rename = "bytes")]
  pub bytes: Option<Vec<u8>>,

  #[serde(rename = "logprob")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,
}
