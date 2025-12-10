use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GeminiModel {
  #[serde(rename = "gemini-3-pro-preview")]
  Gemini3ProPreview,

  #[serde(rename = "gemini-3-pro-image-preview")]
  Gemini3ProImagePreview,

  #[serde(rename = "gemini-2.5-flash")]
  Gemini25Flash,

  #[serde(rename = "gemini-2.5-flash-preview-09-2025")]
  Gemini25FlashPreview092025,

  #[serde(rename = "gemini-2.5-flash-image")]
  Gemini25FlashImage,

  #[serde(rename = "gemini-2.5-flash-native-audio-preview-09-2025")]
  Gemini25FlashNativeAudioPreview092025,

  #[serde(rename = "gemini-2.5-flash-preview-tts")]
  Gemini25FlashPreviewTts,

  #[serde(rename = "gemini-2.5-flash-lite")]
  Gemini25FlashLite,

  #[serde(rename = "gemini-2.5-flash-lite-preview-09-2025")]
  Gemini25FlashLitePreview092025,

  #[serde(rename = "gemini-2.5-pro")]
  Gemini25Pro,

  #[serde(rename = "gemini-2.5-pro-preview-tts")]
  Gemini25ProPreviewTts,

  #[serde(rename = "gemini-2.0-flash")]
  Gemini20Flash,

  #[serde(rename = "gemini-2.0-flash-preview-image-generation")]
  Gemini20FlashPreviewImageGeneration,

  #[serde(rename = "gemini-2.0-flash-lite")]
  Gemini20FlashLite,

  #[serde(rename = "imagen-4.0-generate-001")]
  Imagen40Generate001,

  #[serde(rename = "imagen-4.0-ultra-generate-001")]
  Imagen40UltraGenerate001,

  #[serde(rename = "imagen-4.0-fast-generate-001")]
  Imagen40FastGenerate001,

  #[serde(rename = "imagen-3.0-generate-002")]
  Imagen30Generate002,

  #[serde(rename = "veo-3.1-generate-preview")]
  Veo31GeneratePreview,

  #[serde(rename = "veo-3.1-fast-generate-preview")]
  Veo31FastGeneratePreview,

  #[serde(rename = "veo-3.0-generate-001")]
  Veo30Generate001,

  #[serde(rename = "veo-3.0-fast-generate-001")]
  Veo30FastGenerate001,

  #[serde(rename = "veo-2.0-generate-001")]
  Veo20Generate001,

  #[serde(rename = "lyria-realtime-exp")]
  LyriaRealtimeExp,

  #[serde(rename = "gemini-embedding-001")]
  GeminiEmbedding001,

  #[serde(rename = "gemini-robotics-er-1.5-preview")]
  GeminiRoboticsEr15Preview,

  #[serde(untagged)]
  Other(String),
}

impl GeminiModel {
  pub fn to_str(&self) -> &str {
    match self {
      GeminiModel::Gemini3ProPreview => "gemini-3-pro-preview",
      GeminiModel::Gemini3ProImagePreview => "gemini-3-pro-image-preview",
      GeminiModel::Gemini25Flash => "gemini-2.5-flash",
      GeminiModel::Gemini25FlashPreview092025 => "gemini-2.5-flash-preview-09-2025",
      GeminiModel::Gemini25FlashImage => "gemini-2.5-flash-image",
      GeminiModel::Gemini25FlashNativeAudioPreview092025 => {
        "gemini-2.5-flash-native-audio-preview-09-2025 "
      }
      GeminiModel::Gemini25FlashPreviewTts => "gemini-2.5-flash-preview-tts",
      GeminiModel::Gemini25FlashLite => "gemini-2.5-flash-lite",
      GeminiModel::Gemini25FlashLitePreview092025 => "gemini-2.5-flash-lite-preview-09-2025",
      GeminiModel::Gemini25Pro => "gemini-2.5-pro",
      GeminiModel::Gemini25ProPreviewTts => "gemini-2.5-pro-preview-tts",
      GeminiModel::Gemini20Flash => "gemini-2.0-flash",
      GeminiModel::Gemini20FlashPreviewImageGeneration => {
        "gemini-2.0-flash-preview-image-generation"
      }
      GeminiModel::Gemini20FlashLite => "gemini-2.0-flash-lite",
      GeminiModel::Imagen40Generate001 => "imagen-4.0-generate-001",
      GeminiModel::Imagen40UltraGenerate001 => "imagen-4.0-ultra-generate-001",
      GeminiModel::Imagen40FastGenerate001 => "imagen-4.0-fast-generate-001",
      GeminiModel::Imagen30Generate002 => "imagen-3.0-generate-002",
      GeminiModel::Veo31GeneratePreview => "veo-3.1-generate-preview",
      GeminiModel::Veo31FastGeneratePreview => "veo-3.1-fast-generate-preview",
      GeminiModel::Veo30Generate001 => "veo-3.0-generate-001",
      GeminiModel::Veo30FastGenerate001 => "veo-3.0-fast-generate-001",
      GeminiModel::Veo20Generate001 => "veo-2.0-generate-001",
      GeminiModel::LyriaRealtimeExp => "lyria-realtime-exp",
      GeminiModel::GeminiEmbedding001 => "gemini-embedding-001",
      GeminiModel::GeminiRoboticsEr15Preview => "gemini-robotics-er-1.5-preview",
      GeminiModel::Other(model) => model.as_str(),
    }
  }

  pub fn from_str(value: &str) -> GeminiModel {
    match value {
      "gemini-3-pro-preview" => GeminiModel::Gemini3ProPreview,
      "gemini-3-pro-image-preview" => GeminiModel::Gemini3ProImagePreview,
      "gemini-2.5-flash" => GeminiModel::Gemini25Flash,
      "gemini-2.5-flash-preview-09-2025" => GeminiModel::Gemini25FlashPreview092025,
      "gemini-2.5-flash-image" => GeminiModel::Gemini25FlashImage,
      "gemini-2.5-flash-native-audio-preview-09-2025" => {
        GeminiModel::Gemini25FlashNativeAudioPreview092025
      }
      "gemini-2.5-flash-preview-tts" => GeminiModel::Gemini25FlashPreviewTts,
      "gemini-2.5-flash-lite" => GeminiModel::Gemini25FlashLite,
      "gemini-2.5-flash-lite-preview-09-2025" => GeminiModel::Gemini25FlashLitePreview092025,
      "gemini-2.5-pro" => GeminiModel::Gemini25Pro,
      "gemini-2.5-pro-preview-tts" => GeminiModel::Gemini25ProPreviewTts,
      "gemini-2.0-flash" => GeminiModel::Gemini20Flash,
      "gemini-2.0-flash-preview-image-generation" => {
        GeminiModel::Gemini20FlashPreviewImageGeneration
      }
      "gemini-2.0-flash-lite" => GeminiModel::Gemini20FlashLite,
      "imagen-4.0-generate-001" => GeminiModel::Imagen40Generate001,
      "imagen-4.0-ultra-generate-001" => GeminiModel::Imagen40UltraGenerate001,
      "imagen-4.0-fast-generate-001" => GeminiModel::Imagen40FastGenerate001,
      "imagen-3.0-generate-002" => GeminiModel::Imagen30Generate002,
      "veo-3.1-generate-preview" => GeminiModel::Veo31GeneratePreview,
      "veo-3.1-fast-generate-preview" => GeminiModel::Veo31FastGeneratePreview,
      "veo-3.0-generate-001" => GeminiModel::Veo30Generate001,
      "veo-3.0-fast-generate-001" => GeminiModel::Veo30FastGenerate001,
      "veo-2.0-generate-001" => GeminiModel::Veo20Generate001,
      "lyria-realtime-exp" => GeminiModel::LyriaRealtimeExp,
      "gemini-embedding-001" => GeminiModel::GeminiEmbedding001,
      "gemini-robotics-er-1.5-preview" => GeminiModel::GeminiRoboticsEr15Preview,
      other => GeminiModel::Other(other.to_string()),
    }
  }
}

impl Into<GeminiModel> for &str {
  fn into(self) -> GeminiModel {
    GeminiModel::from_str(self)
  }
}
