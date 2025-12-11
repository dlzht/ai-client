use serde::{Deserialize, Serialize};

use crate::model::DeepSeekChatModel;
use ai_client_common::common::MessageRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekChatRes {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "choices", skip_serializing_if = "Vec::is_empty")]
  choices: Vec<ChoiceData>,

  #[serde(rename = "created")]
  created: i32,

  #[serde(rename = "model")]
  model: DeepSeekChatModel,

  #[serde(rename = "system_fingerprint")]
  system_fingerprint: String,

  #[serde(rename = "object")]
  object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceData {
  #[serde(rename = "finish_reason", skip_serializing_if = "Option::is_none")]
  pub finish_reason: Option<String>,

  #[serde(rename = "index")]
  pub index: i32,

  #[serde(rename = "logprobs", skip_serializing_if = "Option::is_none")]
  pub log_prob: Option<ChoiceLogProb>,

  #[serde(rename = "message")]
  pub message: ChoiceMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceMessage {
  #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,

  #[serde(rename = "reasoning_content", skip_serializing_if = "Option::is_none")]
  pub reasoning_content: Option<String>,

  #[serde(rename = "refusal", skip_serializing_if = "Option::is_none")]
  pub refusal: Option<String>,

  #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
  pub role: Option<MessageRole>,

  #[serde(rename = "tool_calls", skip_serializing_if = "Option::is_none")]
  pub tool_calls: Option<Vec<ChoiceToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceToolCall {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "type")]
  pub kind: String,

  #[serde(rename = "function")]
  pub function: ChoiceFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceFunction {
  #[serde(rename = "arguments")]
  pub arguments: String,

  #[serde(rename = "name")]
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceLogProb {
  #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
  content: Option<Vec<LogProbContent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbContent {
  #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
  pub bytes: Option<Vec<u8>>,

  #[serde(rename = "logprob", skip_serializing_if = "Option::is_none")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,

  #[serde(rename = "top_logprobs")]
  pub top_log_prob: Vec<TopLogProb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogProb {
  #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
  pub bytes: Option<Vec<u8>>,

  #[serde(rename = "logprob", skip_serializing_if = "Option::is_none")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageData {
  #[serde(rename = "completion_tokens")]
  completion_tokens: i32,

  #[serde(rename = "prompt_tokens")]
  prompt_tokens: i32,

  #[serde(rename = "prompt_cache_hit_tokens")]
  prompt_cache_hit_tokens: i32,

  #[serde(rename = "prompt_cache_miss_tokens")]
  prompt_cache_miss_tokens: i32,

  #[serde(rename = "total_tokens")]
  total_tokens: i32,

  #[serde(rename = "completion_tokens_details")]
  completion_tokens_details: Option<CompletionTokensDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTokensDetail {
  #[serde(rename = "reasoning_tokens")]
  reasoning_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeepSeekRes<T> {
  #[serde(rename = "error")]
  Failure(DeepSeekErrRes),
  
  #[serde(untagged)]
  Success(T),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekErrRes {
  #[serde(rename = "message")]
  message: String,

  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "code")]
  code: String,
}


#[cfg(test)]
mod test {
  use crate::response::{DeepSeekChatRes, DeepSeekRes};

  #[test]
  fn test_deserialize_res() {
    let json0 = "{\"error\":{\"message\":\"InsufficientBalance\",\"type\":\"unknown_error\",\"param\":null,\"code\":\"invalid_request_error\"}}";
    assert!(serde_json::from_str::<DeepSeekRes<DeepSeekChatRes>>(json0).is_ok());
    
    let json1 = "{\"id\":\"4a165b25-6790-4a44-8ce5-41e1fbfca940\",\"object\":\"chat.completion\",\"created\":1765431845,\"model\":\"deepseek-chat\",\"choices\":[{\"index\":0,\"message\":{\"role\":\"assistant\",\"content\":\"Hello!HowcanIassistyoutoday?😊\"},\"logprobs\":null,\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":11,\"total_tokens\":21,\"prompt_tokens_details\":{\"cached_tokens\":0},\"prompt_cache_hit_tokens\":0,\"prompt_cache_miss_tokens\":10},\"system_fingerprint\":\"fp_eaab8d114b_prod0820_fp8_kvcache\"}";
    assert!(serde_json::from_str::<DeepSeekRes<DeepSeekChatRes>>(json1).is_ok());
  }
}