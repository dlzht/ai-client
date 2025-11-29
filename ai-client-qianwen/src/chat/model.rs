use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum QianWenChatModel {
  #[serde(rename = "qwen3-max")]
  Qwen3Max,
  
  #[serde(rename = "qwen3-max-2025-09-23")]
  Qwen3Max20250923,
  
  #[serde(rename = "qwen3-max-preview")]
  Qwen3MaxPreview,
  
  #[serde(rename = "qwen-plus")]
  QwenPlus,
  
  #[serde(rename = "qwen-plus-latest")]
  QwenPlusLatest,
  
  #[serde(rename = "qwen-plus-2025-09-11")]
  QwenPlus20250911,
  
  #[serde(rename = "qwen-plus-2025-07-28")]
  QwenPlus20250728,
  
  #[serde(rename = "qwen-plus-2025-07-14")]
  QwenPlus20250714,
  
  #[serde(rename = "qwen-plus-2025-04-28")]
  QwenPlus20250428,
  
  #[serde(rename = "qwen-flash")]
  QwenFlash,
  
  #[serde(rename = "qwen-flash-2025-07-28")]
  QwenFlash20250728,
  
  #[serde(rename = "qwen-turbo")]
  QwenTurbo,
  
  #[serde(rename = "qwen-turbo-latest")]
  QwenTurboLatest,
  
  #[serde(rename = "qwen-turbo-2025-07-15")]
  QwenTurbo20250715,
  
  #[serde(rename = "qwen-turbo-2025-04-28")]
  QwenTurbo20250428,
  
  #[serde(rename = "qwq-plus")]
  QwqPlus,
  
  #[serde(rename = "qwq-plus-latest")]
  QwqPlusLatest,
  
  #[serde(rename = "qwq-plus-2025-03-05")]
  QwqPlus20250305,
  
  #[serde(rename = "qwen-long")]
  QwenLong,
  
  #[serde(rename = "qwen-long-latest")]
  QwenLongLatest,
  
  #[serde(rename = "qwen-long-2025-01-25")]
  QwenLong20250125,
  
  #[serde(rename = "qwen3-omni-flash")]
  Qwen3OmniFlash,
  
  #[serde(rename = "qwen3-omni-flash-2025-09-15")]
  Qwen3OmniFlash20250915,
  
  #[serde(rename = "qwen-omni-turbo")]
  QwenOmniTurbo,
  
  #[serde(rename = "qwen-omni-turbo-latest")]
  QwenOmniTurboLatest,
  
  #[serde(rename = "qwen-omni-turbo-2025-01-19")]
  QwenOmniTurbo20250119,
  
  #[serde(untagged)]
  Other(String)
}

impl QianWenChatModel {
  pub fn to_str(&self) -> &str {
    match self {
      QianWenChatModel::Qwen3Max => "qwen3-max",
      QianWenChatModel::Qwen3Max20250923 => "qwen3-max-2025-09-23",
      QianWenChatModel::Qwen3MaxPreview => "qwen3-max-preview",
      QianWenChatModel::QwenPlus => "qwen-plus",
      QianWenChatModel::QwenPlusLatest => "qwen-plus-latest",
      QianWenChatModel::QwenPlus20250911 => "qwen-plus-2025-09-11",
      QianWenChatModel::QwenPlus20250728 => "qwen-plus-2025-07-28",
      QianWenChatModel::QwenPlus20250714 => "qwen-plus-2025-07-14",
      QianWenChatModel::QwenPlus20250428 => "qwen-plus-2025-04-28",
      QianWenChatModel::QwenFlash => "qwen-flash",
      QianWenChatModel::QwenFlash20250728 => "qwen-flash-2025-07-28",
      QianWenChatModel::QwenTurbo => "qwen-turbo",
      QianWenChatModel::QwenTurboLatest => "qwen-turbo-latest",
      QianWenChatModel::QwenTurbo20250715 => "qwen-turbo-2025-07-15",
      QianWenChatModel::QwenTurbo20250428 => "qwen-turbo-2025-04-28",
      QianWenChatModel::QwqPlus => "qwq-plus",
      QianWenChatModel::QwqPlusLatest => "qwq-plus-latest",
      QianWenChatModel::QwqPlus20250305 => "qwq-plus-2025-03-05",
      QianWenChatModel::QwenLong => "qwen-long",
      QianWenChatModel::QwenLongLatest => "qwen-long-latest",
      QianWenChatModel::QwenLong20250125 => "qwen-long-2025-01-25",
      QianWenChatModel::Qwen3OmniFlash => "qwen3-omni-flash",
      QianWenChatModel::Qwen3OmniFlash20250915 => "qwen3-omni-flash-2025-09-15",
      QianWenChatModel::QwenOmniTurbo => "qwen-omni-turbo",
      QianWenChatModel::QwenOmniTurboLatest => "qwen-omni-turbo-latest",
      QianWenChatModel::QwenOmniTurbo20250119 => "qwen-omni-turbo-2025-01-19",
      QianWenChatModel::Other(s) => s,
    }
  }

  pub fn from_str(value: &str) -> QianWenChatModel {
    match value {
      "qwen3-max" => QianWenChatModel::Qwen3Max,
      "qwen3-max-2025-09-23" => QianWenChatModel::Qwen3Max20250923,
      "qwen3-max-preview" => QianWenChatModel::Qwen3MaxPreview,
      "qwen-plus" => QianWenChatModel::QwenPlus,
      "qwen-plus-latest" => QianWenChatModel::QwenPlusLatest,
      "qwen-plus-2025-09-11" => QianWenChatModel::QwenPlus20250911,
      "qwen-plus-2025-07-28" => QianWenChatModel::QwenPlus20250728,
      "qwen-plus-2025-07-14" => QianWenChatModel::QwenPlus20250714,
      "qwen-plus-2025-04-28" => QianWenChatModel::QwenPlus20250428,
      "qwen-flash" => QianWenChatModel::QwenFlash,
      "qwen-flash-2025-07-28" => QianWenChatModel::QwenFlash20250728,
      "qwen-turbo" => QianWenChatModel::QwenTurbo,
      "qwen-turbo-latest" => QianWenChatModel::QwenTurboLatest,
      "qwen-turbo-2025-07-15" => QianWenChatModel::QwenTurbo20250715,
      "qwen-turbo-2025-04-28" => QianWenChatModel::QwenTurbo20250428,
      "qwq-plus" => QianWenChatModel::QwqPlus,
      "qwq-plus-latest" => QianWenChatModel::QwqPlusLatest,
      "qwq-plus-2025-03-05" => QianWenChatModel::QwqPlus20250305,
      "qwen-long" => QianWenChatModel::QwenLong,
      "qwen-long-latest" => QianWenChatModel::QwenLongLatest,
      "qwen-long-2025-01-25" => QianWenChatModel::QwenLong20250125,
      "qwen3-omni-flash" => QianWenChatModel::Qwen3OmniFlash,
      "qwen3-omni-flash-2025-09-15" => QianWenChatModel::Qwen3OmniFlash20250915,
      "qwen-omni-turbo" => QianWenChatModel::QwenOmniTurbo,
      "qwen-omni-turbo-latest" => QianWenChatModel::QwenOmniTurboLatest,
      "qwen-omni-turbo-2025-01-19" => QianWenChatModel::QwenOmniTurbo20250119,
      _ => QianWenChatModel::Other(value.to_string()),
    }
  }
}

impl Into<QianWenChatModel> for &str {
  fn into(self) -> QianWenChatModel {
    QianWenChatModel::from_str(self)
  }
}