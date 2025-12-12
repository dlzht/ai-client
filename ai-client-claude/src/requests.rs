use ai_client_common::common::MessageRole;
use serde::Serialize;
use serde::Deserialize;

use crate::model::ClaudeModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaudeReq {
  
  #[serde(rename = "model")]
  model: ClaudeModel,
  
  // TODO
  #[serde(rename = "messages")]
  messages: Vec<i32>,
  
  #[serde(rename = "max_tokens")]
  max_tokens: i32,
  
  // TODO
  #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
  metadata: Option<i32>,
  
  // TODO
  #[serde(rename = "service_tier", skip_serializing_if = "Option::is_none")]
  service_tier: Option<String>,
  
  #[serde(rename = "stop_sequences", skip_serializing_if = "Option::is_none")]
  stop_sequences: Option<Vec<String>>,
  
  #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
  stream: Option<bool>,
  
  // TODO
  #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
  system: Option<i32>,
  
  #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,
  
  // TODO
  #[serde(rename = "thinking", skip_serializing_if = "Option::is_none")]
  thinking: Option<i32>,
  
  // TODO
  #[serde(rename = "tool_choice", skip_serializing_if = "Option::is_none")]
  tool_choice: Option<i32>,
  
  // TODO
  #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
  tools: Option<i32>,
  
  #[serde(rename = "top_k", skip_serializing_if = "Option::is_none")]
  top_k: Option<f32>,
  
  #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageParam {
  #[serde(rename = "content")]
  content: MessageContent,
  
  #[serde(rename = "role")]
  role: MessageRole
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
  Text(String),
  Array(Vec<ContentBlockParam>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentBlockParam {
  Text()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextBlockParam {
  #[serde(rename = "text")]
  text: String,
  
  #[serde(rename = "type")]
  kind: String,
  
  #[serde(rename = "cache_control", skip_serializing_if = "Option::is_none")]
  cache_control: Option<CacheControlParam>,
  
  
  citations: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheControlParam {
  #[serde(rename = "type")]
  kind: String,
  
  #[serde(rename = "ttl")]
  ttl: Option<String>
}