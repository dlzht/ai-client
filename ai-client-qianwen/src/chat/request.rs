use ai_client_common::common::MessageRole;
use schemars::Schema;
use serde::{Deserialize, Serialize};

use crate::chat::model::QianWenChatModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenChatReq {
  #[serde(rename = "model")]
  model: QianWenChatModel,

  #[serde(rename = "messages")]
  messages: Vec<MessageParam>,

  #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
  stream: Option<bool>,

  #[serde(rename = "stream_options", skip_serializing_if = "Option::is_none")]
  stream_options: Option<StreamOptionsParam>,

  #[serde(rename = "modalities", skip_serializing_if = "Vec::is_empty")]
  modalities: Vec<OutputModalityParam>,

  #[serde(rename = "audio", skip_serializing_if = "Option::is_none")]
  audio: Option<OutputAudioParam>,

  #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,

  #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,

  #[serde(rename = "top_k", skip_serializing_if = "Option::is_none")]
  top_k: Option<i32>,

  #[serde(rename = "presence_penalty", skip_serializing_if = "Option::is_none")]
  presence_penalty: Option<f32>,

  #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
  response_format: Option<OutputFormatParam>,

  #[serde(rename = "max_input_tokens", skip_serializing_if = "Option::is_none")]
  max_input_tokens: Option<i32>,

  #[serde(rename = "max_tokens", skip_serializing_if = "Option::is_none")]
  max_tokens: Option<i32>,

  #[serde(
    rename = "vl_high_resolution_images",
    skip_serializing_if = "Option::is_none"
  )]
  vl_high_resolution_images: Option<bool>,

  #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
  n: Option<i32>,

  #[serde(rename = "enable_thinking", skip_serializing_if = "Option::is_none")]
  enable_thinking: Option<bool>,

  #[serde(rename = "thinking_budget", skip_serializing_if = "Option::is_none")]
  thinking_budget: Option<i32>,

  #[serde(
    rename = "enable_code_interpreter",
    skip_serializing_if = "Option::is_none"
  )]
  enable_code_interpreter: Option<bool>,

  #[serde(rename = "seed", skip_serializing_if = "Option::is_none")]
  seed: Option<i32>,

  #[serde(rename = "logprobs", skip_serializing_if = "Option::is_none")]
  log_prob: Option<bool>,

  #[serde(rename = "top_logprobs", skip_serializing_if = "Option::is_none")]
  top_log_prob: Option<i32>,

  #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
  stop: Option<StopParam>,

  #[serde(rename = "tools", skip_serializing_if = "Vec::is_empty")]
  tools: Vec<ToolParam>,

  #[serde(rename = "tool_choice", skip_serializing_if = "Option::is_none")]
  tool_choice: Option<ToolChoicesParam>,

  #[serde(
    rename = "parallel_tool_calls",
    skip_serializing_if = "Option::is_none"
  )]
  parallel_tool_calls: Option<bool>,

  #[serde(rename = "enable_search", skip_serializing_if = "Option::is_none")]
  enable_search: Option<bool>,

  #[serde(rename = "search_options", skip_serializing_if = "Option::is_none")]
  search_options: Option<SearchOptionsParam>,

  #[serde(
    rename = "X-DashScope-DataInspection",
    skip_serializing_if = "Option::is_none"
  )]
  data_inspection: Option<String>,
}

impl QianWenChatReq {
  pub fn new(model: impl Into<QianWenChatModel>, messages: Vec<MessageParam>) -> Self {
    Self {
      model: model.into(),
      messages,
      stream: None,
      stream_options: None,
      modalities: Vec::new(),
      audio: None,
      temperature: None,
      top_p: None,
      top_k: None,
      presence_penalty: None,
      response_format: None,
      max_input_tokens: None,
      max_tokens: None,
      vl_high_resolution_images: None,
      n: None,
      enable_thinking: None,
      thinking_budget: None,
      enable_code_interpreter: None,
      seed: None,
      log_prob: None,
      top_log_prob: None,
      stop: None,
      tools: Vec::new(),
      tool_choice: None,
      parallel_tool_calls: None,
      enable_search: None,
      search_options: None,
      data_inspection: None,
    }
  }

  pub fn with_model(mut self, model: impl Into<QianWenChatModel>) -> Self {
    self.model = model.into();
    self
  }

  pub fn with_messages(mut self, messages: Vec<MessageParam>) -> Self {
    self.messages = messages;
    self
  }

  pub fn with_stream(mut self, stream: bool) -> Self {
    self.stream = Some(stream);
    self
  }

  pub fn with_stream_options(mut self, stream_options: StreamOptionsParam) -> Self {
    self.stream_options = Some(stream_options);
    self
  }

  pub fn with_modalities(mut self, modalities: Vec<OutputModalityParam>) -> Self {
    self.modalities = modalities;
    self
  }

  pub fn with_audio(mut self, audio: OutputAudioParam) -> Self {
    self.audio = Some(audio);
    self
  }

  pub fn with_temperature(mut self, temperature: f32) -> Self {
    self.temperature = Some(temperature);
    self
  }

  pub fn with_top_p(mut self, top_p: f32) -> Self {
    self.top_p = Some(top_p);
    self
  }

  pub fn with_top_k(mut self, top_k: i32) -> Self {
    self.top_k = Some(top_k);
    self
  }

  pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
    self.presence_penalty = Some(presence_penalty);
    self
  }

  pub fn with_response_format(mut self, response_format: OutputFormatParam) -> Self {
    self.response_format = Some(response_format);
    self
  }

  pub fn with_max_input_tokens(mut self, max_input_tokens: i32) -> Self {
    self.max_input_tokens = Some(max_input_tokens);
    self
  }

  pub fn with_max_tokens(mut self, max_tokens: i32) -> Self {
    self.max_tokens = Some(max_tokens);
    self
  }

  pub fn with_vl_high_resolution_images(mut self, vl_high_resolution_images: bool) -> Self {
    self.vl_high_resolution_images = Some(vl_high_resolution_images);
    self
  }

  pub fn with_n(mut self, n: i32) -> Self {
    self.n = Some(n);
    self
  }

  pub fn with_enable_thinking(mut self, enable_thinking: bool) -> Self {
    self.enable_thinking = Some(enable_thinking);
    self
  }

  pub fn with_thinking_budget(mut self, thinking_budget: i32) -> Self {
    self.thinking_budget = Some(thinking_budget);
    self
  }

  pub fn with_enable_code_interpreter(mut self, enable_code_interpreter: bool) -> Self {
    self.enable_code_interpreter = Some(enable_code_interpreter);
    self
  }

  pub fn with_seed(mut self, seed: i32) -> Self {
    self.seed = Some(seed);
    self
  }

  pub fn with_log_prob(mut self, log_prob: bool) -> Self {
    self.log_prob = Some(log_prob);
    self
  }

  pub fn with_top_log_prob(mut self, top_log_prob: i32) -> Self {
    self.top_log_prob = Some(top_log_prob);
    self
  }

  pub fn with_stop(mut self, stop: StopParam) -> Self {
    self.stop = Some(stop);
    self
  }

  pub fn with_tools(mut self, tools: Vec<ToolParam>) -> Self {
    self.tools = tools;
    self
  }

  pub fn with_tool_choice(mut self, tool_choice: ToolChoicesParam) -> Self {
    self.tool_choice = Some(tool_choice);
    self
  }

  pub fn with_parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
    self.parallel_tool_calls = Some(parallel_tool_calls);
    self
  }

  pub fn with_enable_search(mut self, enable_search: bool) -> Self {
    self.enable_search = Some(enable_search);
    self
  }

  pub fn with_search_options(mut self, search_options: SearchOptionsParam) -> Self {
    self.search_options = Some(search_options);
    self
  }

  pub fn with_data_inspection(mut self, data_inspection: String) -> Self {
    self.data_inspection = Some(data_inspection);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageParam {
  System(SystemMessage),
  User(UserMessage),
  Assistant(AssistantMessage),
  Tool(ToolMessage),
}

impl MessageParam {
  pub fn new_system_message(content: impl Into<String>) -> Self {
    let message = SystemMessage {
      content: content.into(),
      role: MessageRole::System,
    };
    Self::System(message)
  }

  pub fn new_user_message_with_text(content: impl Into<String>) -> Self {
    let message = UserMessage {
      content: UserMessageContent::Text(content.into()),
      role: MessageRole::User,
    };
    Self::User(message)
  }

  pub fn new_user_message_with_none() -> Self {
    let message = UserMessage {
      content: UserMessageContent::Array(Vec::new()),
      role: MessageRole::User,
    };
    Self::User(message)
  }

  pub fn user_message_with_content_text(
    mut self,
    text: impl Into<String>,
    cache_control: Option<UserMessageCacheControl>,
  ) -> Self {
    self.user_message_transform_content(cache_control);
    if let Self::User(message) = &mut self
      && let UserMessageContent::Array(array) = &mut message.content
    {
      let content = UserMessageContentText {
        text: text.into(),
        kind: "text".to_string(),
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
      };
      array.push(UserMessageContentInner::Text(content));
    }
    self
  }

  pub fn user_message_with_content_image(
    mut self,
    image_url: impl Into<String>,
    min_pixels: Option<i32>,
    max_pixels: Option<i32>,
    cache_control: Option<UserMessageCacheControl>,
  ) -> Self {
    self.user_message_transform_content(cache_control);
    if let Self::User(message) = &mut self
      && let UserMessageContent::Array(array) = &mut message.content
    {
      let content = UserMessageContentImage {
        image_url: UserMessageContentImageInner {
          url: image_url.into(),
        },
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
        min_pixels,
        kind: "image_url".to_string(),
        max_pixels,
      };
      array.push(UserMessageContentInner::ImageUrl(content));
    }
    self
  }

  fn user_message_transform_content(&mut self, cache_control: Option<UserMessageCacheControl>) {
    if let Self::User(message) = self
      && let UserMessageContent::Text(text) = &message.content
    {
      let content = UserMessageContentText {
        text: text.clone(),
        kind: "text".to_string(),
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
      };
      message.content = UserMessageContent::Array(vec![UserMessageContentInner::Text(content)]);
    }
  }

  pub fn user_message_with_content_audio(
    mut self,
    data: impl Into<String>,
    format: impl Into<String>,
    cache_control: Option<UserMessageCacheControl>,
  ) -> Self {
    self.user_message_transform_content(cache_control);
    if let Self::User(message) = &mut self
      && let UserMessageContent::Array(array) = &mut message.content
    {
      let content = UserMessageContentAudio {
        input_audio: UserMessageContentAudioInner {
          data: data.into(),
          format: format.into(),
        },
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
        kind: "input_audio".to_string(),
      };
      array.push(UserMessageContentInner::InputAudio(content));
    }
    self
  }

  pub fn user_message_with_content_video_array(
    mut self,
    video_array: impl IntoIterator<Item = impl Into<String>>,
    cache_control: Option<UserMessageCacheControl>,
  ) -> Self {
    self.user_message_transform_content(cache_control);
    if let Self::User(message) = &mut self
      && let UserMessageContent::Array(array) = &mut message.content
    {
      let content = UserMessageContentVideoArray {
        kind: "video".to_string(),
        video: video_array.into_iter().map(|v| v.into()).collect(),
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
      };
      array.push(UserMessageContentInner::VideoArray(content));
    }
    self
  }

  pub fn user_message_with_content_video_url(
    mut self,
    video_url: impl Into<String>,
    cache_control: Option<UserMessageCacheControl>,
  ) -> Self {
    self.user_message_transform_content(cache_control);
    if let Self::User(message) = &mut self
      && let UserMessageContent::Array(array) = &mut message.content
    {
      let content = UserMessageContentVideo {
        video_url: UserMessageContentVideoInner {
          url: video_url.into(),
        },
        kind: "video_url".to_string(),
        cache_control: cache_control.map(|cache_control| UserMessageContentCache {
          kind: cache_control,
        }),
      };
      array.push(UserMessageContentInner::VideoUrl(content))
    }
    self
  }

  pub fn new_assistant_message() -> Self {
    let message = AssistantMessage {
      content: None,
      role: MessageRole::Assistant,
      partial: None,
      tool_calls: vec![],
    };
    Self::Assistant(message)
  }

  pub fn assistant_message_with_content(mut self, content: impl Into<String>) -> Self {
    if let Self::Assistant(message) = &mut self {
      message.content = Some(content.into());
    }
    self
  }

  pub fn assistant_message_with_partial(mut self, partial: bool) -> Self {
    if let Self::Assistant(message) = &mut self {
      message.partial = Some(partial);
    }
    self
  }

  pub fn assistant_message_with_tool_call(
    mut self,
    id: impl Into<String>,
    index: i32,
    name: impl Into<String>,
    arguments: impl Into<String>,
  ) -> Self {
    if let Self::Assistant(message) = &mut self {
      let tool_call = AssistantMessageToolCall {
        id: id.into(),
        index,
        function: AssistantMessageToolCallFunction {
          name: name.into(),
          arguments: arguments.into(),
        },
        kind: "function".to_string(),
      };
      message.tool_calls = vec![tool_call];
    }
    self
  }

  pub fn assistant_message_with_tool_calls(
    mut self,
    tool_calls: impl IntoIterator<Item = (impl Into<String>, i32, impl Into<String>, impl Into<String>)>,
  ) -> Self {
    if let Self::Assistant(message) = &mut self {
      let tool_calls = tool_calls
        .into_iter()
        .map(|(id, index, name, arguments)| AssistantMessageToolCall {
          id: id.into(),
          index,
          function: AssistantMessageToolCallFunction {
            name: name.into(),
            arguments: arguments.into(),
          },
          kind: "function".to_string(),
        })
        .collect();
      message.tool_calls = tool_calls;
    }
    self
  }

  pub fn new_tool_message(content: impl Into<String>, tool_call_id: impl Into<String>) -> Self {
    let message = ToolMessage {
      content: content.into(),
      role: MessageRole::Tool,
      tool_call_id: tool_call_id.into(),
    };
    Self::Tool(message)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SystemMessage {
  #[serde(rename = "content")]
  content: String,

  #[serde(rename = "role")]
  role: MessageRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessage {
  #[serde(rename = "content")]
  content: UserMessageContent,

  #[serde(rename = "role")]
  role: MessageRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum UserMessageContent {
  Text(String),
  Array(Vec<UserMessageContentInner>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum UserMessageCacheControl {
  #[serde(rename = "ephemeral")]
  Ephemeral,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum UserMessageContentInner {
  Text(UserMessageContentText),
  ImageUrl(UserMessageContentImage),
  InputAudio(UserMessageContentAudio),
  VideoArray(UserMessageContentVideoArray),
  VideoUrl(UserMessageContentVideo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentText {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "url")]
  text: String,

  #[serde(rename = "cache_control")]
  cache_control: Option<UserMessageContentCache>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentImage {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "image_url")]
  image_url: UserMessageContentImageInner,

  #[serde(rename = "cache_control", skip_serializing_if = "Option::is_none")]
  cache_control: Option<UserMessageContentCache>,

  #[serde(rename = "min_pixels", skip_serializing_if = "Option::is_none")]
  min_pixels: Option<i32>,

  #[serde(rename = "max_pixels", skip_serializing_if = "Option::is_none")]
  max_pixels: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentImageInner {
  #[serde(rename = "url")]
  url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentAudio {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "input_audio")]
  input_audio: UserMessageContentAudioInner,

  #[serde(rename = "cache_control")]
  cache_control: Option<UserMessageContentCache>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentAudioInner {
  #[serde(rename = "data")]
  data: String,

  #[serde(rename = "format")]
  format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentVideoArray {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "video")]
  video: Vec<String>,

  #[serde(rename = "cache_control", skip_serializing_if = "Option::is_none")]
  cache_control: Option<UserMessageContentCache>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentVideo {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "input_audio")]
  video_url: UserMessageContentVideoInner,

  #[serde(rename = "cache_control", skip_serializing_if = "Option::is_none")]
  cache_control: Option<UserMessageContentCache>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentVideoInner {
  #[serde(rename = "url")]
  url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageContentCache {
  #[serde(rename = "type")]
  kind: UserMessageCacheControl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessage {
  #[serde(rename = "content")]
  content: Option<String>,

  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "partial", skip_serializing_if = "Option::is_none")]
  partial: Option<bool>,

  #[serde(rename = "tool_calls", skip_serializing_if = "Vec::is_empty")]
  tool_calls: Vec<AssistantMessageToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCall {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "index")]
  index: i32,

  #[serde(rename = "function")]
  function: AssistantMessageToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCallFunction {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "arguments")]
  arguments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolMessage {
  #[serde(rename = "content")]
  content: String,

  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "tool_call_id")]
  tool_call_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOptionsParam {
  #[serde(rename = "include_usage", skip_serializing_if = "Option::is_none")]
  include_usage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OutputModalityParam {
  #[serde(rename = "text")]
  Text,

  #[serde(rename = "audio")]
  Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputAudioParam {
  #[serde(rename = "voice")]
  voice: OutputVoiceType,

  #[serde(rename = "format")]
  format: OutputVoiceFormat,
}

impl OutputAudioParam {
  pub fn new(voice: OutputVoiceType) -> Self {
    Self {
      voice,
      format: OutputVoiceFormat::Wav,
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum OutputVoiceType {
  #[serde(rename = "Cherry")]
  Cherry,

  #[serde(rename = "Ethan")]
  Ethan,

  #[serde(rename = "Nofish")]
  NoFish,

  #[serde(rename = "Jennifer")]
  Jennifer,

  #[serde(rename = "Ryan")]
  Ryan,

  #[serde(rename = "Katerina")]
  Katerina,

  #[serde(rename = "Elias")]
  Elias,

  #[serde(rename = "Jada")]
  Jada,

  #[serde(rename = "Dylan")]
  Dylan,

  #[serde(rename = "Sunny")]
  Sunny,

  #[serde(rename = "Li")]
  Li,

  #[serde(rename = "Marcus")]
  Marcus,

  #[serde(rename = "Roy")]
  Roy,

  #[serde(rename = "Peter")]
  Peter,

  #[serde(rename = "Rocky")]
  Rocky,

  #[serde(rename = "Kiki")]
  Kiki,

  #[serde(rename = "Eric")]
  Eric,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum OutputVoiceFormat {
  #[serde(rename = "wav")]
  Wav,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OutputFormatParam {
  Text(OutputFormatText),
  JsonObject(OutputFormatJsonObject),
}

impl OutputFormatParam {
  pub fn new_text() -> Self {
    OutputFormatParam::Text(OutputFormatText {
      kind: "text".to_string(),
    })
  }

  pub fn new_json_object() -> Self {
    OutputFormatParam::JsonObject(OutputFormatJsonObject {
      kind: "json_object".to_string(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OutputFormatText {
  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OutputFormatJsonObject {
  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopParam {
  Text(String),
  Array(Vec<String>),
}

impl StopParam {
  pub fn new_text(text: impl Into<String>) -> Self {
    StopParam::Text(text.into())
  }

  pub fn new_array(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {
    let texts = texts.into_iter().map(|s| s.into()).collect();
    StopParam::Array(texts)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolParam {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "function")]
  function: ToolFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolFunction {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description")]
  description: String,

  #[serde(rename = "parameters")]
  parameters: Schema,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolChoicesParam {
  #[serde(rename = "auto")]
  Auto,

  #[serde(rename = "none")]
  None,

  #[serde(untagged)]
  Force(ToolChoicesForce),
}

impl ToolChoicesParam {
  pub fn new_force(name: impl Into<String>) -> Self {
    ToolChoicesParam::Force(ToolChoicesForce {
      kind: "function".to_string(),
      function: ToolChoicesForceInner { name: name.into() },
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoicesForce {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "function")]
  function: ToolChoicesForceInner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoicesForceInner {
  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchOptionsParam {
  #[serde(rename = "forced_search", skip_serializing_if = "Option::is_none")]
  forced_search: Option<bool>,

  #[serde(rename = "search_strategy", skip_serializing_if = "Option::is_none")]
  search_strategy: Option<SearchOptionStrategy>,

  #[serde(
    rename = "enable_search_extension",
    skip_serializing_if = "Option::is_none"
  )]
  enable_search_extension: Option<bool>,
}

impl SearchOptionsParam {
  pub fn new() -> Self {
    Self {
      forced_search: None,
      search_strategy: None,
      enable_search_extension: None,
    }
  }

  pub fn with_forced_search(mut self, forced_search: bool) -> Self {
    self.forced_search = Some(forced_search);
    self
  }

  pub fn with_search_strategy(mut self, search_strategy: SearchOptionStrategy) -> Self {
    self.search_strategy = Some(search_strategy);
    self
  }

  pub fn with_enable_search_extension(mut self, enable_search_extension: bool) -> Self {
    self.enable_search_extension = Some(enable_search_extension);
    self
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum SearchOptionStrategy {
  #[serde(rename = "turbo")]
  Turbo,

  #[serde(rename = "max")]
  Max,

  #[serde(rename = "agent")]
  Agent,
}
