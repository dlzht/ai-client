use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum MessageRole {
  #[serde(rename = "developer")]
  Developer,

  #[serde(rename = "system")]
  System,

  #[serde(rename = "user")]
  User,

  #[serde(rename = "assistant")]
  Assistant,

  #[serde(rename = "tool")]
  Tool,
}