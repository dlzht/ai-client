use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClaudeModel {
  
  #[serde(untagged)]
  Other(String)
}