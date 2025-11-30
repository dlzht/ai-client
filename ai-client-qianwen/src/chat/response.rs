use ai_client_common::common::MessageRole;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRes {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "created")]
  created: i32,

  #[serde(rename = "model")]
  model: String,

  #[serde(rename = "object")]
  object: String,

  #[serde(rename = "service_tier")]
  service_tier: Option<String>,

  #[serde(rename = "system_fingerprint")]
  system_fingerprint: Option<String>,

  #[serde(rename = "usage")]
  usage: Option<UsageData>,

  #[serde(rename = "choices")]
  choices: Option<Vec<ChoiceData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceData {
  #[serde(rename = "finish_reason")]
  pub finish_reason: Option<String>,

  #[serde(rename = "index")]
  pub index: i32,

  #[serde(rename = "logprobs")]
  pub log_prob: Option<ChoiceLogProb>,

  #[serde(rename = "message")]
  pub message: ChoiceMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceMessage {
  #[serde(rename = "content")]
  pub content: String,

  #[serde(rename = "reasoning_content")]
  pub reasoning_content: Option<String>,

  #[serde(rename = "refusal")]
  pub refusal: Option<String>,

  #[serde(rename = "role")]
  pub role: Option<MessageRole>,

  // #[serde(rename = "audio")]
  // audio: Option<i32>,

  // #[serde(rename = "function_call")]
  // function_call: Option<i32>,
  #[serde(rename = "tool_calls")]
  pub tool_calls: Option<Vec<ChoiceToolCall>>,
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
  // #[serde(rename = "accepted_prediction_tokens")]
  // pub accepted_prediction_tokens: i32,
  #[serde(rename = "audio_tokens")]
  pub audio_tokens: Option<i32>,

  #[serde(rename = "reasoning_tokens")]
  pub reasoning_tokens: Option<i32>,

  #[serde(rename = "text_tokens")]
  pub text_tokens: Option<i32>,
  // #[serde(rename = "rejected_prediction_tokens")]
  // pub rejected_prediction_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptTokensDetails {
  #[serde(rename = "audio_tokens")]
  pub audio_tokens: Option<i32>,

  #[serde(rename = "cached_tokens")]
  pub cached_tokens: Option<i32>,

  #[serde(rename = "text_tokens")]
  pub text_tokens: Option<i32>,

  #[serde(rename = "image_tokens")]
  pub image_tokens: Option<i32>,

  #[serde(rename = "video_tokens")]
  pub video_tokens: Option<i32>,

  #[serde(rename = "cache_creation")]
  pub cache_creation: Option<CacheCreationTokenDetail>,

  #[serde(rename = "cache_creation_input_tokens")]
  pub cache_creation_input_tokens: Option<i32>,

  #[serde(rename = "cache_type")]
  pub cache_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheCreationTokenDetail {
  #[serde(rename = "ephemeral_5m_input_tokens")]
  ephemeral_5m_input_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenChatChunkData {
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
  pub log_prob: Option<ChoiceLogProb>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkChoiceDelta {
  #[serde(rename = "content")]
  pub content: String,

  #[serde(rename = "reasoning_content")]
  pub reasoning_content: Option<String>,

  #[serde(rename = "refusal")]
  pub refusal: Option<String>,

  #[serde(rename = "role")]
  pub role: Option<MessageRole>,

  #[serde(rename = "tool_calls", default)]
  pub tool_calls: Option<Vec<ChoiceToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceToolCall {
  #[serde(rename = "index")]
  pub index: i32,

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
  #[serde(rename = "content")]
  content: Option<Vec<ChoiceLogProbData>>,
  // #[serde(rename = "refusal")]
  // refusal: Option<Vec<ChunkChoiceLogProbData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceLogProbData {
  #[serde(rename = "bytes")]
  pub bytes: Option<Vec<u8>>,

  #[serde(rename = "logprob")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,

  #[serde(rename = "top_logprobs")]
  pub top_log_prob: Option<Vec<ChoiceTopLogProb>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceTopLogProb {
  #[serde(rename = "bytes")]
  pub bytes: Vec<u8>,

  #[serde(rename = "logprob")]
  pub log_prob: Option<f32>,

  #[serde(rename = "token")]
  pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorRes {
  #[serde(rename = "error")]
  pub error: ErrorData,

  #[serde(rename = "request_id")]
  pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorData {
  #[serde(rename = "message")]
  pub message: String,

  #[serde(rename = "type")]
  pub kind: Option<String>,

  #[serde(rename = "param")]
  pub param: Option<String>,

  #[serde(rename = "code")]
  pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum QianWenChatRes<T> {
  Failure(ErrorRes),
  Success(T),
}

#[cfg(test)]
mod test {
  use crate::chat::response::{ChatCompletionRes, QianWenChatChunkData, QianWenChatRes};

  #[test]
  fn test_deserialize_res() {
    let json0 = "{\"choices\":[{\"message\":{\"role\":\"assistant\",\"content\":\"我是阿里云开发的一款超大规模语言模型，我叫通义千问。\"},\"finish_reason\":\"stop\",\"index\":0,\"logprobs\":null}],\"object\":\"chat.completion\",\"usage\":{\"prompt_tokens\":3019,\"completion_tokens\":104,\"total_tokens\":3123,\"prompt_tokens_details\":{\"cached_tokens\":2048}},\"created\":1735120033,\"system_fingerprint\":null,\"model\":\"qwen-plus\",\"id\":\"chatcmpl-6ada9ed2-7f33-9de2-8bb0-78bd4035025a\"}";
    assert!(serde_json::from_str::<QianWenChatRes<ChatCompletionRes>>(json0).is_ok());

    let json1 = "{\"error\": {\"message\": \"you must provide a messages parameter\",\"type\": \"invalid_request_error\",\"param\": \"message\",\"code\": \"missing_required_parameter\"},\"request_id\": \"chatcmpl-026b188f-77c4-453d-a64f-60e7b444faa7\"}";
    assert!(serde_json::from_str::<QianWenChatRes<ChatCompletionRes>>(json1).is_ok());

    let json2 = "{\"id\":\"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57\",\"choices\":[{\"delta\":{\"content\":\"\",\"function_call\":null,\"refusal\":null,\"role\":\"assistant\",\"tool_calls\":null},\"finish_reason\":null,\"index\":0,\"logprobs\":null}],\"created\":1735113344,\"model\":\"qwen-plus\",\"object\":\"chat.completion.chunk\",\"service_tier\":null,\"system_fingerprint\":null,\"usage\":null}";
    assert!(serde_json::from_str::<QianWenChatRes<QianWenChatChunkData>>(json2).is_ok());

    let json3 = "{\"id\":\"chatcmpl-e30f5ae7-3063-93c4-90fe-beb5f900bd57\",\"choices\":[],\"created\":1735113344,\"model\":\"qwen-plus\",\"object\":\"chat.completion.chunk\",\"service_tier\":null,\"system_fingerprint\":null,\"usage\":{\"completion_tokens\":17,\"prompt_tokens\":22,\"total_tokens\":39,\"completion_tokens_details\":null,\"prompt_tokens_details\":{\"audio_tokens\":null,\"cached_tokens\":0}}}";
    assert!(serde_json::from_str::<QianWenChatRes<QianWenChatChunkData>>(json3).is_ok());
  }
}
