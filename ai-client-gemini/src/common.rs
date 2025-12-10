use std::collections::HashMap;

use ai_client_common::common::MessageRole;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HarmCategory {
  #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
  Derogatory,

  #[serde(rename = "HARM_CATEGORY_TOXICITY")]
  Toxicity,

  #[serde(rename = "HARM_CATEGORY_SEXUAL")]
  Sexual,

  #[serde(rename = "HARM_CATEGORY_MEDICAL")]
  Medical,

  #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
  Dangerous,

  #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
  Harassment,

  #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
  HateSpeech,

  #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
  SexuallyExplicit,

  #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
  DangerousContent,

  #[serde(rename = "HARM_CATEGORY_CIVIC_INTEGRITY")]
  CivicIntegrity,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HarmBlockThreshold {
  #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
  LowAndAbove,

  #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
  MediumAndAbove,

  #[serde(rename = "BLOCK_ONLY_HIGH")]
  OnlyHigh,

  #[serde(rename = "BLOCK_NONE")]
  None,

  #[serde(rename = "OFF")]
  Off,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HarmProbability {
  #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "NEGLIGIBLE")]
  Negligible,

  #[serde(rename = "LOW")]
  Low,

  #[serde(rename = "MEDIUM")]
  Medium,

  #[serde(rename = "HIGH")]
  High,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modality {
  #[serde(rename = "MODALITY_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "TEXT")]
  Text,

  #[serde(rename = "IMAGE")]
  Image,

  #[serde(rename = "AUDIO")]
  Audio,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThinkingLevel {
  #[serde(rename = "THINKING_LEVEL_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "LOW")]
  Low,

  #[serde(rename = "HIGH")]
  High,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AspectRatio {
  #[serde(rename = "1:1")]
  W1H1,

  #[serde(rename = "9:16")]
  W9H16,

  #[serde(rename = "16:9")]
  W16H9,

  #[serde(rename = "3:4")]
  W3H4,

  #[serde(rename = "4:3")]
  W4H3,

  #[serde(rename = "3:2")]
  W3H2,

  #[serde(rename = "2:3")]
  W2H3,

  #[serde(rename = "5:4")]
  W5H4,

  #[serde(rename = "4:5")]
  W4H5,

  #[serde(rename = "21:9")]
  W21H9,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageSize {
  #[serde(rename = "1K")]
  R1k,

  #[serde(rename = "2K")]
  R2K,

  #[serde(rename = "4K")]
  R4k,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaResolution {
  #[serde(rename = "MEDIA_RESOLUTION_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "MEDIA_RESOLUTION_LOW")]
  Low,

  #[serde(rename = "MEDIA_RESOLUTION_MEDIUM")]
  Medium,

  #[serde(rename = "MEDIA_RESOLUTION_HIGH")]
  High,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DynamicRetrievalMode {
  #[serde(rename = "MODE_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "MODE_DYNAMIC")]
  Dynamic,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComputerUserEnvironment {
  #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "ENVIRONMENT_BROWSER")]
  Browser,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CodeLanguage {
  #[serde(rename = "LANGUAGE_UNSPECIFIED")]
  LanguageUnspecified,

  #[serde(rename = "PYTHON")]
  Python,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionCallingMode {
  #[serde(rename = "MODE_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "AUTO")]
  Auto,

  #[serde(rename = "ANY")]
  Any,

  #[serde(rename = "NONE")]
  None,

  #[serde(rename = "VALIDATED")]
  Validated,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionBehavior {
  #[serde(rename = "UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "BLOCKING")]
  Blocking,

  #[serde(rename = "NON_BLOCKING")]
  NonBlocking,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FinishReason {
  #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "STOP")]
  Stop,

  #[serde(rename = "MAX_TOKEN")]
  MaxToken,

  #[serde(rename = "SAFETY")]
  Safety,

  #[serde(rename = "RECITATION")]
  Recitation,

  #[serde(rename = "LANGUAGE")]
  Language,

  #[serde(rename = "OTHER")]
  OTHER,

  #[serde(rename = "BLOCKLIST")]
  Blocklist,

  #[serde(rename = "PROHIBITED_CONTENT")]
  ProhibitedContent,

  #[serde(rename = "SPII")]
  SensitiveInformation,

  #[serde(rename = "MALFORMED_FUNCTION_CALL")]
  MalformedFunctionCall,

  #[serde(rename = "IMAGE_SAFETY")]
  ImageSafety,

  #[serde(rename = "IMAGE_PROHIBITED_CONTENT")]
  ImageProhibitedContent,

  #[serde(rename = "IMAGE_OTHER")]
  ImageOther,

  #[serde(rename = "NO_IMAGE")]
  NoImage,

  #[serde(rename = "IMAGE_RECITATION")]
  ImageRecitation,

  #[serde(rename = "UNEXPECTED_TOOL_CALL")]
  UnexpectedToolCall,

  #[serde(rename = "TOO_MANY_TOOL_CALLS")]
  TooManyToolCalls,

  #[serde(rename = "MISSING_THOUGHT_SIGNATURE")]
  MissingThoughtSignature,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
  #[serde(rename = "parts")]
  parts: Vec<ContentPart>,

  #[serde(rename = "role")]
  role: MessageRole,
}

impl Content {
  pub fn new_user_content(parts: Vec<ContentPart>) -> Self {
    Content {
      parts,
      role: MessageRole::User,
    }
  }

  pub fn new_model_content(parts: Vec<ContentPart>) -> Self {
    Content {
      parts,
      role: MessageRole::Model,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentPart {
  #[serde(rename = "thought", skip_serializing_if = "Option::is_none")]
  thought: Option<bool>,

  #[serde(rename = "thoughtSignature", skip_serializing_if = "Option::is_none")]
  thought_signature: Option<String>,

  #[serde(
    rename = "partMetadata",
    skip_serializing_if = "HashMap::is_empty",
    default
  )]
  part_metadata: HashMap<String, Value>,

  #[serde(flatten)]
  data: PartData,

  #[serde(flatten)]
  metadata: Option<PartDataMetadata>,
}

impl ContentPart {
  pub fn new_with_text_data(text: impl Into<String>) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::Text(text.into()),
      metadata: None,
    }
  }
  pub fn new_with_inline_data(mime: impl Into<String>, data: impl Into<String>) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::InlineData(PartInlineData {
        mime: mime.into(),
        data: data.into(),
      }),
      metadata: None,
    }
  }

  pub fn new_with_function_call_data(
    id: Option<impl Into<String>>,
    name: impl Into<String>,
    args: Option<Value>,
  ) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::FunctionCall(PartFunctionCall {
        id: id.map(|id| id.into()),
        name: name.into(),
        args,
      }),
      metadata: None,
    }
  }

  pub fn new_with_function_response_data(
    id: Option<impl Into<String>>,
    name: impl Into<String>,
    response: Value,
    parts: Option<impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>>,
    will_continue: Option<bool>,
    scheduling: Option<FunctionScheduling>,
  ) -> Self {
    let parts = parts
      .map(|parts| {
        parts
          .into_iter()
          .map(|(key, value)| {
            FunctionResponsePart::InlineData(PartInlineData {
              mime: key.into(),
              data: value.into(),
            })
          })
          .collect::<Vec<_>>()
      })
      .unwrap_or(Vec::new());
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::FunctionResponse(PartFunctionResponse {
        id: id.map(|id| id.into()),
        name: name.into(),
        response,
        parts,
        will_continue,
        scheduling,
      }),
      metadata: None,
    }
  }

  pub fn new_with_file_data(mime: Option<impl Into<String>>, file_url: impl Into<String>) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::FileData(PartFileData {
        mime: mime.map(|m| m.into()),
        file_uri: file_url.into(),
      }),
      metadata: None,
    }
  }

  pub fn new_with_executable_code(language: CodeLanguage, code: impl Into<String>) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::ExecutableCode(PartExecutableCode {
        language,
        code: code.into(),
      }),
      metadata: None,
    }
  }

  pub fn new_with_code_execution(
    outcome: CodeExecutionOutcome,
    output: Option<impl Into<String>>,
  ) -> Self {
    ContentPart {
      thought: None,
      thought_signature: None,
      part_metadata: HashMap::new(),
      data: PartData::CodeExecutionResult(PartCodeExecutionResult {
        outcome,
        output: output.map(|o| o.into()),
      }),
      metadata: None,
    }
  }

  pub fn with(mut self, thought: bool) -> Self {
    self.thought = Some(thought);
    self
  }

  pub fn with_thought_signature(mut self, thought_signature: impl Into<String>) -> Self {
    self.thought_signature = Some(thought_signature.into());
    self
  }

  pub fn with_part_metadata(mut self, part_metadata: HashMap<String, Value>) -> Self {
    self.part_metadata = part_metadata;
    self
  }

  pub fn with_metadata(mut self, metadata: PartDataMetadata) -> Self {
    self.metadata = Some(metadata);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum PartData {
  #[serde(rename = "text")]
  Text(String),

  #[serde(rename = "inlineData")]
  InlineData(PartInlineData),

  #[serde(rename = "functionCall")]
  FunctionCall(PartFunctionCall),

  #[serde(rename = "functionResponse")]
  FunctionResponse(PartFunctionResponse),

  #[serde(rename = "fileData")]
  FileData(PartFileData),

  #[serde(rename = "executableCode")]
  ExecutableCode(PartExecutableCode),

  #[serde(rename = "codeExecutionResult")]
  CodeExecutionResult(PartCodeExecutionResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PartDataMetadata {
  #[serde(rename = "videoMetadata")]
  Video(VideoMetadata),
}

impl PartDataMetadata {
  pub fn new_video_metadata(
    start_offset: Option<impl Into<String>>,
    end_offset: Option<impl Into<String>>,
    fps: Option<f32>,
  ) -> Self {
    PartDataMetadata::Video(VideoMetadata {
      start_offset: start_offset.map(|s| s.into()),
      end_offset: end_offset.map(|e| e.into()),
      fps,
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartInlineData {
  #[serde(rename = "mimeType")]
  mime: String,

  #[serde(rename = "data")]
  data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartFunctionCall {
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  id: Option<String>,

  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
  args: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartFunctionResponse {
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  id: Option<String>,

  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "response")]
  response: Value,

  #[serde(rename = "parts", skip_serializing_if = "Vec::is_empty", default)]
  parts: Vec<FunctionResponsePart>,

  #[serde(rename = "willContinue", skip_serializing_if = "Option::is_none")]
  will_continue: Option<bool>,

  #[serde(rename = "scheduling", skip_serializing_if = "Option::is_none")]
  scheduling: Option<FunctionScheduling>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionScheduling {
  #[serde(rename = "SCHEDULING_UNSPECIFIED")]
  SchedulingUnspecified,

  #[serde(rename = "SILENT")]
  Silent,

  #[serde(rename = "WHEN_IDLE")]
  WhenIdle,

  #[serde(rename = "INTERRUPT")]
  Interrupt,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionResponsePart {
  #[serde(rename = "inlineData")]
  InlineData(PartInlineData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartFileData {
  #[serde(rename = "mimeType")]
  mime: Option<String>,

  #[serde(rename = "fileUri")]
  file_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartExecutableCode {
  #[serde(rename = "language")]
  language: CodeLanguage,

  #[serde(rename = "code")]
  code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartCodeExecutionResult {
  #[serde(rename = "outcome")]
  outcome: CodeExecutionOutcome,

  #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
  output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VideoMetadata {
  #[serde(rename = "start_offset", skip_serializing_if = "Option::is_none")]
  start_offset: Option<String>,

  #[serde(rename = "end_offset", skip_serializing_if = "Option::is_none")]
  end_offset: Option<String>,

  #[serde(rename = "fps", skip_serializing_if = "Option::is_none")]
  fps: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CodeExecutionOutcome {
  #[serde(rename = "OUTCOME_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "OUTCOME_OK")]
  Ok,

  #[serde(rename = "OUTCOME_Failed")]
  Failed,

  #[serde(rename = "OUTCOME_DEADLINE_EXCEEDED")]
  DeadlineExceeded,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UrlRetrievalStatus {
  #[serde(rename = "URL_RETRIEVAL_STATUS_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "URL_RETRIEVAL_STATUS_SUCCESS")]
  Success,

  #[serde(rename = "URL_RETRIEVAL_STATUS_ERROR")]
  Error,

  #[serde(rename = "URL_RETRIEVAL_STATUS_PAYWALL")]
  Paywall,

  #[serde(rename = "URL_RETRIEVAL_STATUS_UNSAFE")]
  Unsafe,

  #[serde(untagged)]
  Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockReason {
  #[serde(rename = "BLOCK_REASON_UNSPECIFIED")]
  Unspecified,

  #[serde(rename = "SAFETY")]
  Safety,

  #[serde(rename = "OTHER")]
  OTHER,

  #[serde(rename = "BLOCKLIST")]
  Blocklist,

  #[serde(rename = "PROHIBITED_CONTENT")]
  ProhibitedContent,

  #[serde(rename = "Image_Safety")]
  ImageSafety,

  #[serde(untagged)]
  Other(String),
}
