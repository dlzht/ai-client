use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum OpenAiChatModel {
  #[serde(rename = "gpt-5.1")]
  Gpt51,
  
  #[serde(rename = "o4-mini")]
  O4Mini,
  
  #[serde(rename = "o3")]
  O3,
  
  #[serde(rename = "o3-mini")]
  O3Mini,
  
  #[serde(rename = "o1")]
  O1,
  
  #[serde(rename = "o1-mini")]
  O1Mini,
  
  #[serde(rename = "o1-pro")]
  O1Pro,
  
  #[serde(rename = "gpt-4.1")]
  Gpt41,
  
  #[serde(rename = "gpt-5")]
  Gpt5,
  
  #[serde(rename = "gpt-5-mini")]
  Gpt5Mini,
  
  #[serde(rename = "gpt-5-nano")]
  Gpt5Nano,
  
  #[serde(rename = "gpt-5-chat-latest")]
  Gpt5ChatLatest,
  
  #[serde(rename = "gpt-4o")]
  Gpt4O,
  
  #[serde(rename = "chatgpt-4o-latest")]
  Chatgpt4OLatest,
  
  #[serde(rename = "gpt-4o-audio-preview")]
  Gpt4OAudioPreview,
  
  #[serde(rename = "gpt-4.1-mini")]
  Gpt41Mini,
  
  #[serde(rename = "gpt-4.1-nano")]
  Gpt41Nano,
  
  #[serde(rename = "gpt-4o-mini")]
  Gpt4OMini,
  
  #[serde(rename = "gpt-4o-mini-audio-preview")]
  Gpt4OMiniAudioPreview,
  
  #[serde(rename = "gpt-4o-realtime-preview")]
  Gpt4oRealtimePreview,
  
  #[serde(rename = "gpt-4o-mini-realtime-preview")]
  Gpt4oMiniRealtimePreview,
  
  #[serde(rename = "gpt-4-turbo")]
  Gpt4Turbo,
  
  #[serde(rename = "gpt-4")]
  Gpt4,
  
  #[serde(rename = "gpt-3.5-turbo")]
  Gpt35Turbo,
  
  #[serde(rename = "gpt-3.5-turbo-instruct")]
  Gpt35TurboInstruct,
  
  #[serde(rename = "gpt-4o-search-preview")]
  Gpt4OSearchPreview,
  
  #[serde(rename = "gpt-4o-mini-search-preview")]
  Gpt4OMiniSearchPreview,
  
  #[serde(untagged)]
  Other(String),
}

impl OpenAiChatModel {
  pub fn to_str(&self) -> &str {
    match self {
      OpenAiChatModel::Gpt51 => "gpt-5.1",
      OpenAiChatModel::O4Mini => "o4-mini",
      OpenAiChatModel::O3 => "o3",
      OpenAiChatModel::O3Mini => "o3-mini",
      OpenAiChatModel::O1 => "o1",
      OpenAiChatModel::O1Mini => "o1-mini",
      OpenAiChatModel::O1Pro => "o1-pro",
      OpenAiChatModel::Gpt41 => "gpt-4.1",
      OpenAiChatModel::Gpt5 => "gpt-5",
      OpenAiChatModel::Gpt5Mini => "gpt-5-mini",
      OpenAiChatModel::Gpt5Nano => "gpt-5-nano",
      OpenAiChatModel::Gpt5ChatLatest => "gpt-5-chat-latest",
      OpenAiChatModel::Gpt4O => "gpt-4o",
      OpenAiChatModel::Chatgpt4OLatest => "chatgpt-4o-latest",
      OpenAiChatModel::Gpt4OAudioPreview => "gpt-4o-audio-preview",
      OpenAiChatModel::Gpt41Mini => "gpt-4.1-mini",
      OpenAiChatModel::Gpt41Nano => "gpt-4.1-nano",
      OpenAiChatModel::Gpt4OMini => "gpt-4o-mini",
      OpenAiChatModel::Gpt4OMiniAudioPreview => "gpt-4o-mini-audio-preview",
      OpenAiChatModel::Gpt4oRealtimePreview => "gpt-4o-realtime-preview",
      OpenAiChatModel::Gpt4oMiniRealtimePreview => "gpt-4o-mini-realtime-preview",
      OpenAiChatModel::Gpt4Turbo => "gpt-4-turbo",
      OpenAiChatModel::Gpt4 => "gpt-4",
      OpenAiChatModel::Gpt35Turbo => "gpt-3.5-turbo",
      OpenAiChatModel::Gpt35TurboInstruct => "gpt-3.5-turbo-instruct",
      OpenAiChatModel::Gpt4OSearchPreview => "gpt-4o-search-preview",
      OpenAiChatModel::Gpt4OMiniSearchPreview => "gpt-4o-mini-search-preview",
      OpenAiChatModel::Other(s) => s.as_str(),
    }
  }

  pub fn from_str(value: &str) -> OpenAiChatModel {
    match value {
      "gpt-5.1" => OpenAiChatModel::Gpt51,
      "o4-mini" => OpenAiChatModel::O4Mini,
      "o3" => OpenAiChatModel::O3,
      "o3-mini" => OpenAiChatModel::O3Mini,
      "o1" => OpenAiChatModel::O1,
      "o1-mini" => OpenAiChatModel::O1Mini,
      "o1-pro" => OpenAiChatModel::O1Pro,
      "gpt-4.1" => OpenAiChatModel::Gpt41,
      "gpt-5" => OpenAiChatModel::Gpt5,
      "gpt-5-mini" => OpenAiChatModel::Gpt5Mini,
      "gpt-5-nano" => OpenAiChatModel::Gpt5Nano,
      "gpt-5-chat-latest" => OpenAiChatModel::Gpt5ChatLatest,
      "gpt-4o" => OpenAiChatModel::Gpt4O,
      "chatgpt-4o-latest" => OpenAiChatModel::Chatgpt4OLatest,
      "gpt-4o-audio-preview" => OpenAiChatModel::Gpt4OAudioPreview,
      "gpt-4.1-mini" => OpenAiChatModel::Gpt41Mini,
      "gpt-4.1-nano" => OpenAiChatModel::Gpt41Nano,
      "gpt-4o-mini" => OpenAiChatModel::Gpt4OMini,
      "gpt-4o-mini-audio-preview" => OpenAiChatModel::Gpt4OMiniAudioPreview,
      "gpt-4o-realtime-preview" => OpenAiChatModel::Gpt4oRealtimePreview,
      "gpt-4o-mini-realtime-preview" => OpenAiChatModel::Gpt4oMiniRealtimePreview,
      "gpt-4-turbo" => OpenAiChatModel::Gpt4Turbo,
      "gpt-4" => OpenAiChatModel::Gpt4,
      "gpt-3.5-turbo" => OpenAiChatModel::Gpt35Turbo,
      "gpt-3.5-turbo-instruct" => OpenAiChatModel::Gpt35TurboInstruct,
      "gpt-4o-search-preview" => OpenAiChatModel::Gpt4OSearchPreview,
      "gpt-4o-mini-search-preview" => OpenAiChatModel::Gpt4OMiniSearchPreview,
      _ => OpenAiChatModel::Other(value.to_string()),
    }
  }
}

impl From<&str> for OpenAiChatModel {
  fn from(value: &str) -> Self {
    OpenAiChatModel::from_str(value)
  }
}
