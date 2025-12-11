use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeepSeekChatModel {
  
  #[serde(rename = "DeepSeek-V3.2")]
  DeepSeekV32,
  
  #[serde(untagged)]
  Other(String),
}
