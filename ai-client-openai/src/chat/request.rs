use std::collections::HashMap;

use schemars::Schema;
use serde::{Deserialize, Serialize};
use ai_client_common::common::MessageRole;
use crate::chat::model::OpenAiChatModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChatReq {
  #[serde(rename = "messages")]
  messages: Vec<MessageParam>,

  #[serde(rename = "model")]
  model: OpenAiChatModel,

  #[serde(rename = "audio", skip_serializing_if = "Option::is_none")]
  audio: Option<AudioParam>,

  #[serde(
    rename = "frequency_penalty",
    skip_serializing_if = "Option::is_none"
  )]
  frequency_penalty: Option<f32>,

  // #[serde(rename = "function_call")]
  // function_call: Option<u32>,
  //
  // #[serde(rename = "functions")]
  // functions: Option<u32>,
  #[serde(
    rename = "logit_bias",
    skip_serializing_if = "HashMap::is_empty"
  )]
  logit_bias: LogitBiasParam,

  #[serde(
    rename = "logprobs",
    skip_serializing_if = "Option::is_none"
  )]
  log_prob: Option<bool>,

  #[serde(
    rename = "max_completion_tokens",
    skip_serializing_if = "Option::is_none"
  )]
  max_completion_tokens: Option<i32>,

  // #[serde(rename = "max_tokens")]
  // max_tokens: Option<i32>,
  #[serde(
    rename = "metadata",
    skip_serializing_if = "HashMap::is_empty"
  )]
  metadata: MetadataParam,

  #[serde(
    rename = "modalities",
    skip_serializing_if = "Vec::is_empty"
  )]
  modalities: Vec<OutputModalityParam>,

  #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
  n: Option<i32>,

  #[serde(
    rename = "parallel_tool_calls",
    skip_serializing_if = "Option::is_none"
  )]
  parallel_tool_calls: Option<bool>,

  #[serde(
    rename = "prediction",
    skip_serializing_if = "Option::is_none"
  )]
  prediction: Option<PredictionParam>,

  #[serde(
    rename = "presence_penalty",
    skip_serializing_if = "Option::is_none"
  )]
  presence_penalty: Option<f32>,

  #[serde(
    rename = "prompt_cache_key",
    skip_serializing_if = "Option::is_none"
  )]
  prompt_cache_key: Option<String>,

  #[serde(
    rename = "prompt_cache_retention",
    skip_serializing_if = "Option::is_none"
  )]
  prompt_cache_retention: Option<String>,

  #[serde(
    rename = "reasoning_effort",
    skip_serializing_if = "Option::is_none"
  )]
  reasoning_effort: Option<String>,

  #[serde(
    rename = "response_format",
    skip_serializing_if = "Option::is_none"
  )]
  response_format: Option<OutputFormatParam>,

  #[serde(
    rename = "safety_identifier",
    skip_serializing_if = "Option::is_none"
  )]
  safety_identifier: Option<String>,

  // #[serde(rename = "seed")]
  // seed: Option<i32>,
  #[serde(
    rename = "service_tier",
    skip_serializing_if = "Option::is_none"
  )]
  service_tier: Option<String>,

  #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
  stop: Option<StopParam>,

  #[serde(rename = "store", skip_serializing_if = "Option::is_none")]
  store: Option<bool>,

  #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
  stream: Option<bool>,

  #[serde(
    rename = "stream_options",
    skip_serializing_if = "Option::is_none"
  )]
  stream_options: Option<StreamOptionsParam>,

  #[serde(
    rename = "temperature",
    skip_serializing_if = "Option::is_none"
  )]
  temperature: Option<f32>,

  #[serde(
    rename = "tool_choice",
    skip_serializing_if = "Option::is_none"
  )]
  tool_choice: Option<ToolChoiceParam>,

  #[serde(rename = "tools", skip_serializing_if = "Vec::is_empty")]
  tools: Vec<ToolsParam>,

  #[serde(
    rename = "top_logprobs",
    skip_serializing_if = "Option::is_none"
  )]
  top_log_prob: Option<i32>,

  #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,

  // #[serde(rename = "user")]
  // user: Option<String>,
  #[serde(
    rename = "verbosity",
    skip_serializing_if = "Option::is_none"
  )]
  verbosity: Option<VerbosityParam>,

  #[serde(
    rename = "web_search_options",
    skip_serializing_if = "Option::is_none"
  )]
  web_search_options: Option<WebSearchOptionsParam>,
}

impl OpenAiChatReq {
  pub fn new(model: impl Into<OpenAiChatModel>, messages: Vec<MessageParam>) -> Self {
    OpenAiChatReq {
      messages,
      model: model.into(),
      audio: None,
      frequency_penalty: None,
      logit_bias: LogitBiasParam::new(),
      log_prob: None,
      max_completion_tokens: None,
      metadata: MetadataParam::new(),
      modalities: Vec::new(),
      n: None,
      parallel_tool_calls: None,
      prediction: None,
      presence_penalty: None,
      prompt_cache_key: None,
      prompt_cache_retention: None,
      reasoning_effort: None,
      response_format: None,
      safety_identifier: None,
      service_tier: None,
      stop: None,
      store: None,
      stream: None,
      stream_options: None,
      temperature: None,
      tool_choice: None,
      tools: Vec::new(),
      top_log_prob: None,
      top_p: None,
      verbosity: None,
      web_search_options: None,
    }
  }

  pub fn with_audio(mut self, audio: AudioParam) -> Self {
    self.audio = Some(audio);
    self
  }

  pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
    self.frequency_penalty = Some(frequency_penalty);
    self
  }

  pub fn with_logit_bias(mut self, logit_bias: LogitBiasParam) -> Self {
    self.logit_bias = logit_bias;
    self
  }

  pub fn with_log_prob(mut self, log_prob: bool) -> Self {
    self.log_prob = Some(log_prob);
    self
  }

  pub fn with_max_completion_tokens(mut self, max_completion_tokens: i32) -> Self {
    self.max_completion_tokens = Some(max_completion_tokens);
    self
  }

  pub fn with_metadata(mut self, metadata: MetadataParam) -> Self {
    self.metadata = metadata;
    self
  }

  pub fn with_modalities(mut self, modalities: Vec<OutputModalityParam>) -> Self {
    self.modalities = modalities;
    self
  }

  pub fn with_n(mut self, n: i32) -> Self {
    self.n = Some(n);
    self
  }

  pub fn with_parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
    self.parallel_tool_calls = Some(parallel_tool_calls);
    self
  }

  pub fn with_prediction(mut self, prediction: PredictionParam) -> Self {
    self.prediction = Some(prediction);
    self
  }

  pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
    self.presence_penalty = Some(presence_penalty);
    self
  }

  pub fn with_prompt_cache_key(mut self, prompt_cache_key: String) -> Self {
    self.prompt_cache_key = Some(prompt_cache_key);
    self
  }

  pub fn with_prompt_cache_retention(mut self, prompt_cache_retention: String) -> Self {
    self.prompt_cache_retention = Some(prompt_cache_retention);
    self
  }

  pub fn with_reasoning_effort(mut self, reasoning_effort: String) -> Self {
    self.reasoning_effort = Some(reasoning_effort);
    self
  }

  pub fn with_response_format(mut self, response_format: OutputFormatParam) -> Self {
    self.response_format = Some(response_format);
    self
  }

  pub fn with_safety_identifier(mut self, safety_identifier: String) -> Self {
    self.safety_identifier = Some(safety_identifier);
    self
  }

  pub fn with_service_tier(mut self, service_tier: String) -> Self {
    self.service_tier = Some(service_tier);
    self
  }

  pub fn with_stop(mut self, stop: StopParam) -> Self {
    self.stop = Some(stop);
    self
  }

  pub fn with_store(mut self, store: bool) -> Self {
    self.store = Some(store);
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

  pub fn with_temperature(mut self, temperature: f32) -> Self {
    self.temperature = Some(temperature);
    self
  }

  pub fn with_tool_choice(mut self, tool_choice: ToolChoiceParam) -> Self {
    self.tool_choice = Some(tool_choice);
    self
  }

  pub fn with_tools(mut self, tools: Vec<ToolsParam>) -> Self {
    self.tools = tools;
    self
  }

  pub fn with_top_log_prob(mut self, top_log_prob: i32) -> Self {
    self.top_log_prob = Some(top_log_prob);
    self
  }

  pub fn with_top_p(mut self, top_p: f32) -> Self {
    self.top_p = Some(top_p);
    self
  }

  pub fn with_verbosity(mut self, verbosity: VerbosityParam) -> Self {
    self.verbosity = Some(verbosity);
    self
  }

  pub fn with_web_search_options(mut self, web_search_options: WebSearchOptionsParam) -> Self {
    self.web_search_options = Some(web_search_options);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageParam {
  Developer(DeveloperMessage),
  System(SystemMessage),
  User(UserMessage),
  Assistant(AssistantMessage),
  Tool(ToolMessage),
}

impl MessageParam {
  pub fn new_developer_message_with_text(text: impl Into<String>) -> MessageParam {
    let message = DeveloperMessage {
      role: MessageRole::Developer,
      name: None,
      content: DeveloperMessageContent::Text(text.into()),
    };
    MessageParam::Developer(message)
  }

  pub fn new_developer_message_with_part(
    kind: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let part = DeveloperMessageContentPart {
      text: text.into(),
      kind: kind.into(),
    };
    let message = DeveloperMessage {
      role: MessageRole::Developer,
      name: None,
      content: DeveloperMessageContent::Part(vec![part]),
    };
    MessageParam::Developer(message)
  }

  pub fn new_developer_message_with_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, text)| DeveloperMessageContentPart {
        text: text.into(),
        kind: kind.into(),
      })
      .collect();
    let message = DeveloperMessage {
      role: MessageRole::Developer,
      name: None,
      content: DeveloperMessageContent::Part(parts),
    };
    MessageParam::Developer(message)
  }

  pub fn developer_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let MessageParam::Developer(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn new_system_message_with_text(text: impl Into<String>) -> MessageParam {
    let message = SystemMessage {
      role: MessageRole::System,
      name: None,
      content: SystemMessageContent::Text(text.into()),
    };
    MessageParam::System(message)
  }

  pub fn new_system_message_with_part(
    kind: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let part = SystemMessageContentPart {
      text: text.into(),
      kind: kind.into(),
    };
    let message = SystemMessage {
      role: MessageRole::System,
      name: None,
      content: SystemMessageContent::Part(vec![part]),
    };
    MessageParam::System(message)
  }

  pub fn new_system_message_with_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, text)| SystemMessageContentPart {
        text: text.into(),
        kind: kind.into(),
      })
      .collect();
    let message = SystemMessage {
      role: MessageRole::System,
      name: None,
      content: SystemMessageContent::Part(parts),
    };
    MessageParam::System(message)
  }

  pub fn system_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let MessageParam::System(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn new_user_message_with_text(text: impl Into<String>) -> MessageParam {
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Text(text.into()),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_text_part(
    kind: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let part = UserMessageTextPart {
      text: text.into(),
      kind: kind.into(),
    };
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(vec![UserMessageContentPart::Text(part)]),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_text_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, text)| {
        UserMessageContentPart::Text(UserMessageTextPart {
          text: text.into(),
          kind: kind.into(),
        })
      })
      .collect();
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(parts),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_image_part(
    kind: impl Into<String>,
    image_url: impl Into<String>,
    image_detail: Option<String>,
  ) -> MessageParam {
    let part = UserMessageImagePart {
      kind: kind.into(),
      image: UserMessageImagePartInner {
        url: image_url.into(),
        detail: image_detail,
      },
    };
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(vec![UserMessageContentPart::Image(part)]),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_image_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>, Option<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, image_url, image_detail)| {
        UserMessageContentPart::Image(UserMessageImagePart {
          image: UserMessageImagePartInner {
            url: image_url.into(),
            detail: image_detail,
          },
          kind: kind.into(),
        })
      })
      .collect();
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(parts),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_audio_part(
    audio_data: impl Into<String>,
    audio_format: UserMessageAudioFormat,
  ) -> MessageParam {
    let part = UserMessageAudioPart {
      kind: "input_audio".to_string(),
      audio: UserMessageAudioPartInner {
        data: audio_data.into(),
        format: audio_format,
      },
    };
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(vec![UserMessageContentPart::Audio(part)]),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_audio_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, UserMessageAudioFormat)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(audio_data, audio_format)| {
        UserMessageContentPart::Audio(UserMessageAudioPart {
          audio: UserMessageAudioPartInner {
            data: audio_data.into(),
            format: audio_format,
          },
          kind: "input_audio".to_string(),
        })
      })
      .collect();
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(parts),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_file_part(
    file_id: Option<String>,
    file_data: Option<String>,
    file_name: Option<String>,
  ) -> MessageParam {
    let part = UserMessageFilePart {
      file: UserMessageFilePartInner {
        data: file_data,
        id: file_id,
        name: file_name,
      },
      kind: "file".to_string(),
    };
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(vec![UserMessageContentPart::File(part)]),
    };
    MessageParam::User(message)
  }

  pub fn new_user_message_with_file_parts(
    parts: impl IntoIterator<Item = (Option<String>, Option<String>, Option<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(file_id, file_data, file_name)| {
        UserMessageContentPart::File(UserMessageFilePart {
          file: UserMessageFilePartInner {
            id: file_id,
            data: file_data,
            name: file_name,
          },
          kind: "file".to_string(),
        })
      })
      .collect();
    let message = UserMessage {
      role: MessageRole::User,
      name: None,
      content: UserMessageContent::Part(parts),
    };
    MessageParam::User(message)
  }

  pub fn user_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let MessageParam::User(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn new_assistant_message_with_text(text: impl Into<String>) -> MessageParam {
    let message = AssistantMessage {
      role: MessageRole::Assistant,
      name: None,
      audio: None,
      content: AssistantMessageContent::Text(text.into()),
      refusal: None,
      tool_calls: Vec::new(),
    };
    MessageParam::Assistant(message)
  }

  pub fn new_assistant_message_with_text_part(
    kind: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let parts = AssistantMessageContentPart::Text(AssistantMessageContentTextPart {
      text: text.into(),
      kind: kind.into(),
    });
    let message = AssistantMessage {
      role: MessageRole::Assistant,
      name: None,
      audio: None,
      content: AssistantMessageContent::Part(vec![parts]),
      refusal: None,
      tool_calls: Vec::new(),
    };
    MessageParam::Assistant(message)
  }

  pub fn new_assistant_message_with_text_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, text)| {
        AssistantMessageContentPart::Text(AssistantMessageContentTextPart {
          text: text.into(),
          kind: kind.into(),
        })
      })
      .collect();
    let message = AssistantMessage {
      role: MessageRole::Assistant,
      name: None,
      audio: None,
      content: AssistantMessageContent::Part(parts),
      refusal: None,
      tool_calls: Vec::new(),
    };
    MessageParam::Assistant(message)
  }

  pub fn new_assistant_message_with_refusal_part(
    kind: impl Into<String>,
    refusal: impl Into<String>,
  ) -> MessageParam {
    let parts = AssistantMessageContentPart::Refusal(AssistantMessageContentRefusalPart {
      refusal: refusal.into(),
      kind: kind.into(),
    });
    let message = AssistantMessage {
      role: MessageRole::Assistant,
      name: None,
      audio: None,
      content: AssistantMessageContent::Part(vec![parts]),
      refusal: None,
      tool_calls: Vec::new(),
    };
    MessageParam::Assistant(message)
  }

  pub fn new_assistant_message_with_refusal_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, refusal)| {
        AssistantMessageContentPart::Refusal(AssistantMessageContentRefusalPart {
          refusal: refusal.into(),
          kind: kind.into(),
        })
      })
      .collect();
    let message = AssistantMessage {
      role: MessageRole::Assistant,
      name: None,
      audio: None,
      content: AssistantMessageContent::Part(parts),
      refusal: None,
      tool_calls: Vec::new(),
    };
    MessageParam::Assistant(message)
  }

  pub fn assistant_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let MessageParam::Assistant(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn assistant_message_with_audio(mut self, audio_id: impl Into<String>) -> Self {
    if let MessageParam::Assistant(message) = &mut self {
      message.audio = Some(AssistantMessageAudio {
        id: audio_id.into(),
      });
    }
    self
  }

  pub fn assistant_message_with_refusal(mut self, refusal: impl Into<String>) -> Self {
    if let MessageParam::Assistant(message) = &mut self {
      message.refusal = Some(refusal.into());
    }
    self
  }

  pub fn assistant_message_with_function_tool_call(
    mut self,
    function_name: impl Into<String>,
    function_arguments: impl Into<String>,
    tool_call_id: impl Into<String>,
  ) -> Self {
    let tool_call = AssistantMessageToolCallFunction {
      function: AssistantMessageToolCallFunctionInner {
        arguments: function_arguments.into(),
        name: function_name.into(),
      },
      id: tool_call_id.into(),
      kind: "function".to_string(),
    };
    if let MessageParam::Assistant(message) = &mut self {
      message.tool_calls = vec![AssistantMessageToolCall::Function(tool_call)];
    }
    self
  }

  pub fn assistant_message_with_function_tool_calls(
    mut self,
    functions: impl IntoIterator<Item = (impl Into<String>, impl Into<String>, impl Into<String>)>,
  ) -> Self {
    let tool_calls = functions
      .into_iter()
      .map(|(function_name, function_arguments, tool_call_id)| {
        AssistantMessageToolCall::Function(AssistantMessageToolCallFunction {
          function: AssistantMessageToolCallFunctionInner {
            arguments: function_arguments.into(),
            name: function_name.into(),
          },
          id: tool_call_id.into(),
          kind: "function".to_string(),
        })
      })
      .collect();
    if let MessageParam::Assistant(message) = &mut self {
      message.tool_calls = tool_calls;
    }
    self
  }

  pub fn assistant_message_with_custom_tool_call(
    mut self,
    custom_name: impl Into<String>,
    custom_input: impl Into<String>,
    tool_call_id: impl Into<String>,
  ) -> Self {
    let tool_call = AssistantMessageToolCallCustom {
      custom: AssistantMessageToolCallCustomInner {
        input: custom_input.into(),
        name: custom_name.into(),
      },
      id: tool_call_id.into(),
      kind: "custom".to_string(),
    };
    if let MessageParam::Assistant(message) = &mut self {
      message.tool_calls = vec![AssistantMessageToolCall::Custom(tool_call)];
    }
    self
  }

  pub fn assistant_message_with_custom_tool_calls(
    mut self,
    customs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>, impl Into<String>)>,
  ) -> Self {
    let tool_calls = customs
      .into_iter()
      .map(|(custom_name, custom_input, tool_call_id)| {
        AssistantMessageToolCall::Custom(AssistantMessageToolCallCustom {
          custom: AssistantMessageToolCallCustomInner {
            input: custom_input.into(),
            name: custom_name.into(),
          },
          id: tool_call_id.into(),
          kind: "custom".to_string(),
        })
      })
      .collect();
    if let MessageParam::Assistant(message) = &mut self {
      message.tool_calls = tool_calls;
    }
    self
  }

  pub fn new_tool_message_with_text(
    tool_call_id: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let message = ToolMessage {
      role: MessageRole::Tool,
      tool_call_id: tool_call_id.into(),
      content: ToolMessageContent::Text(text.into()),
    };
    MessageParam::Tool(message)
  }

  pub fn new_tool_message_with_text_part(
    tool_call_id: impl Into<String>,
    kind: impl Into<String>,
    text: impl Into<String>,
  ) -> MessageParam {
    let part = ToolMessageContentPart {
      text: text.into(),
      kind: kind.into(),
    };
    let message = ToolMessage {
      role: MessageRole::Tool,
      tool_call_id: tool_call_id.into(),
      content: ToolMessageContent::Part(vec![part]),
    };
    MessageParam::Tool(message)
  }

  pub fn new_tool_message_with_text_parts(
    tool_call_id: impl Into<String>,
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> MessageParam {
    let parts = parts
      .into_iter()
      .map(|(kind, text)| ToolMessageContentPart {
        text: text.into(),
        kind: kind.into(),
      })
      .collect();
    let message = ToolMessage {
      role: MessageRole::Tool,
      tool_call_id: tool_call_id.into(),
      content: ToolMessageContent::Part(parts),
    };
    MessageParam::Tool(message)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DeveloperMessage {
  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[serde(rename = "content")]
  content: DeveloperMessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum DeveloperMessageContent {
  Text(String),
  Part(Vec<DeveloperMessageContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DeveloperMessageContentPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SystemMessage {
  role: MessageRole,
  name: Option<String>,
  content: SystemMessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum SystemMessageContent {
  Text(String),
  Part(Vec<SystemMessageContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SystemMessageContentPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessage {
  role: MessageRole,
  name: Option<String>,
  content: UserMessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum UserMessageContent {
  Text(String),
  Part(Vec<UserMessageContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum UserMessageContentPart {
  Text(UserMessageTextPart),
  Image(UserMessageImagePart),
  Audio(UserMessageAudioPart),
  File(UserMessageFilePart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageTextPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageImagePart {
  #[serde(rename = "image_url")]
  image: UserMessageImagePartInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageImagePartInner {
  #[serde(rename = "url")]
  url: String,

  #[serde(rename = "detail")]
  detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageAudioPart {
  #[serde(rename = "input_audio")]
  audio: UserMessageAudioPartInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserMessageAudioFormat {
  #[serde(rename = "wav")]
  Wav,

  #[serde(rename = "mp3")]
  Mp3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageAudioPartInner {
  #[serde(rename = "data")]
  data: String,

  #[serde(rename = "format")]
  format: UserMessageAudioFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageFilePart {
  #[serde(rename = "file")]
  file: UserMessageFilePartInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessageFilePartInner {
  #[serde(rename = "file_data")]
  data: Option<String>,

  #[serde(rename = "file_id")]
  id: Option<String>,

  #[serde(rename = "filename")]
  name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessage {
  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "name")]
  name: Option<String>,

  #[serde(rename = "audio")]
  audio: Option<AssistantMessageAudio>,

  #[serde(rename = "content")]
  content: AssistantMessageContent,

  // function_call: Option<AssistantMessageFunctionCall>,
  #[serde(rename = "refusal")]
  refusal: Option<String>,

  #[serde(rename = "tool_calls")]
  tool_calls: Vec<AssistantMessageToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageAudio {
  #[serde(rename = "id")]
  id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum AssistantMessageContent {
  Text(String),
  Part(Vec<AssistantMessageContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum AssistantMessageContentPart {
  Text(AssistantMessageContentTextPart),
  Refusal(AssistantMessageContentRefusalPart),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageContentTextPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageContentRefusalPart {
  #[serde(rename = "refusal")]
  refusal: String,

  #[serde(rename = "type")]
  kind: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub(crate) struct AssistantMessageFunctionCall {
//   #[serde(rename = "arguments")]
//   arguments: String,
//
//   #[serde(rename = "name")]
//   name: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum AssistantMessageToolCall {
  Function(AssistantMessageToolCallFunction),
  Custom(AssistantMessageToolCallCustom),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCallFunction {
  #[serde(rename = "function")]
  function: AssistantMessageToolCallFunctionInner,

  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCallFunctionInner {
  #[serde(rename = "arguments")]
  arguments: String,

  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCallCustom {
  #[serde(rename = "custom")]
  custom: AssistantMessageToolCallCustomInner,

  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessageToolCallCustomInner {
  #[serde(rename = "input")]
  input: String,

  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolMessage {
  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "tool_call_id")]
  tool_call_id: String,

  #[serde(rename = "content")]
  content: ToolMessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum ToolMessageContent {
  Text(String),
  Part(Vec<ToolMessageContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolMessageContentPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioParam {
  #[serde(rename = "format")]
  format: AudioOutputType,

  #[serde(rename = "voice")]
  voice: AudioVoiceModel,
}

impl AudioParam {
  pub fn new(format: AudioOutputType, voice: AudioVoiceModel) -> Self {
    Self { format, voice }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum AudioVoiceModel {
  #[serde(rename = "alloy")]
  Alloy,

  #[serde(rename = "ash")]
  Ash,

  #[serde(rename = "ballad")]
  Ballad,

  #[serde(rename = "coral")]
  Coral,

  #[serde(rename = "echo")]
  Echo,

  #[serde(rename = "fable")]
  Fable,

  #[serde(rename = "nova")]
  Nova,

  #[serde(rename = "onyx")]
  Onyx,

  #[serde(rename = "sage")]
  Sage,

  #[serde(rename = "shimmer")]
  Shimmer,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum AudioOutputType {
  #[serde(rename = "wav")]
  Wav,

  #[serde(rename = "mp3")]
  Mp3,

  #[serde(rename = "flac")]
  Flac,

  #[serde(rename = "opus")]
  Opus,

  #[serde(rename = "pcm16")]
  Pcm16,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum OutputModalityParam {
  #[serde(rename = "text")]
  Text,

  #[serde(rename = "audio")]
  Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionParam {
  #[serde(rename = "content")]
  content: PredictionContent,

  #[serde(rename = "type")]
  kind: PredictionType,
}

impl PredictionParam {
  pub fn new_with_text(text: impl Into<String>) -> Self {
    PredictionParam {
      content: PredictionContent::Text(text.into()),
      kind: PredictionType::Content,
    }
  }

  pub fn new_with_part(kind: impl Into<String>, text: impl Into<String>) -> Self {
    let part = PredictionContentPart {
      text: text.into(),
      kind: kind.into(),
    };
    PredictionParam {
      content: PredictionContent::Part(vec![part]),
      kind: PredictionType::Content,
    }
  }

  pub fn new_with_parts(
    parts: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> Self {
    let parts = parts
      .into_iter()
      .map(|(text, kind)| PredictionContentPart {
        text: text.into(),
        kind: kind.into(),
      })
      .collect();
    PredictionParam {
      content: PredictionContent::Part(parts),
      kind: PredictionType::Content,
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum PredictionType {
  #[serde(rename = "content")]
  Content,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum PredictionContent {
  Text(String),
  Part(Vec<PredictionContentPart>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PredictionContentPart {
  #[serde(rename = "text")]
  text: String,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OutputFormatParam {
  Text(OutputFormatText),
  JsonObject(OutputFormatJsonObject),
  JsonSchema(OutputFormatJsonSchema),
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

  pub fn new_json_schema(
    name: impl Into<String>,
    description: Option<impl Into<String>>,
    strict: Option<bool>,
    schema: Schema,
  ) -> Self {
    OutputFormatParam::JsonSchema(OutputFormatJsonSchema {
      kind: "json_schema".to_string(),
      schema: OutputFormatJsonSchemaInner {
        name: name.into(),
        description: description.map(|s| s.into()),
        strict,
        schema,
      },
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
pub(crate) struct OutputFormatJsonSchema {
  #[serde(rename = "json_schema")]
  schema: OutputFormatJsonSchemaInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct OutputFormatJsonSchemaInner {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description")]
  description: Option<String>,

  #[serde(rename = "strict")]
  strict: Option<bool>,

  #[serde(rename = "schema")]
  schema: Schema,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
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
pub struct StreamOptionsParam {
  #[serde(
    rename = "include_obfuscation",
    skip_serializing_if = "Option::is_none"
  )]
  include_obfuscation: Option<bool>,

  #[serde(
    rename = "include_usage",
    skip_serializing_if = "Option::is_none"
  )]
  include_usage: Option<bool>,
}

impl StreamOptionsParam {
  pub fn new() -> Self {
    StreamOptionsParam {
      include_obfuscation: None,
      include_usage: None,
    }
  }

  pub fn with_include_obfuscation(mut self, include_obfuscation: bool) -> Self {
    self.include_obfuscation = Some(include_obfuscation);
    self
  }

  pub fn with_include_usage(mut self, include_usage: bool) -> Self {
    self.include_usage = Some(include_usage);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolChoiceParam {
  #[serde(rename = "none")]
  None,

  #[serde(rename = "auto")]
  Auto,

  #[serde(rename = "required")]
  Required,

  #[serde(untagged)]
  Allowed(ToolChoiceAllowed),

  #[serde(untagged)]
  Function(ToolChoiceFunction),

  #[serde(untagged)]
  Custom(ToolChoiceCustom),
}

impl ToolChoiceParam {
  pub fn new_function_tool(name: impl Into<String>) -> Self {
    ToolChoiceParam::Function(ToolChoiceFunction {
      function: ToolChoiceFunctionInner { name: name.into() },
      kind: "function".to_string(),
    })
  }

  pub fn new_custom_tool(name: impl Into<String>) -> Self {
    ToolChoiceParam::Custom(ToolChoiceCustom {
      custom: ToolChoiceCustomInner { name: name.into() },
      kind: "custom".to_string(),
    })
  }

  pub fn new_allowed_tools(
    mode: ToolChoiceAllowedMode,
    tools: impl IntoIterator<Item = impl Into<String>>,
  ) -> Self {
    let tools = tools
      .into_iter()
      .map(|t| ToolChoiceAllowedTool {
        function: ToolChoiceAllowedToolInner { name: t.into() },
        kind: "function".to_string(),
      })
      .collect();
    ToolChoiceParam::Allowed(ToolChoiceAllowed { mode, tools })
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum ToolChoiceAllowedMode {
  #[serde(rename = "auto")]
  Auto,

  #[serde(rename = "required")]
  Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceAllowed {
  #[serde(rename = "mode")]
  mode: ToolChoiceAllowedMode,

  #[serde(rename = "tools")]
  tools: Vec<ToolChoiceAllowedTool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceAllowedTool {
  #[serde(rename = "function")]
  function: ToolChoiceAllowedToolInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceAllowedToolInner {
  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceFunction {
  #[serde(rename = "function")]
  function: ToolChoiceFunctionInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceFunctionInner {
  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceCustom {
  #[serde(rename = "custom")]
  custom: ToolChoiceCustomInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolChoiceCustomInner {
  #[serde(rename = "name")]
  name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolsParam {
  Function(FunctionTool),
  Custom(CustomTool),
}

impl ToolsParam {
  pub fn new_function_tool(name: impl Into<String>) -> Self {
    let tool = FunctionTool {
      function: FunctionToolFunction {
        name: name.into(),
        description: None,
        parameters: None,
        strict: None,
      },
      kind: "function".to_string(),
    };
    ToolsParam::Function(tool)
  }

  pub fn function_tool_with_description(mut self, description: impl Into<String>) -> Self {
    if let ToolsParam::Function(tool) = &mut self {
      tool.function.description = Some(description.into());
    }
    self
  }

  pub fn function_tool_with_parameters(mut self, parameters: Schema) -> Self {
    if let ToolsParam::Function(tool) = &mut self {
      tool.function.parameters = Some(parameters);
    }
    self
  }

  pub fn function_tool_with_strict(mut self, strict: bool) -> Self {
    if let ToolsParam::Function(tool) = &mut self {
      tool.function.strict = Some(strict);
    }
    self
  }

  pub fn new_custom_tool(name: impl Into<String>) -> Self {
    let tool = CustomTool {
      custom: CustomToolInner {
        name: name.into(),
        description: None,
        format: None,
      },
      kind: "custom".to_string(),
    };
    ToolsParam::Custom(tool)
  }

  pub fn custom_tool_with_description(mut self, description: impl Into<String>) -> Self {
    if let ToolsParam::Custom(tool) = &mut self {
      tool.custom.description = Some(description.into());
    }
    self
  }

  pub fn custom_tool_with_format_text(mut self) -> Self {
    if let ToolsParam::Custom(tool) = &mut self {
      let format = CustomToolFormat::Text(CustomToolFormatText {
        kind: "text".to_string(),
      });
      tool.custom.format = Some(format);
    }
    self
  }

  pub fn custom_tool_with_grammar_format(
    mut self,
    definition: impl Into<String>,
    syntax: CustomToolGrammarSyntax,
  ) -> Self {
    if let ToolsParam::Custom(tool) = &mut self {
      let format = CustomToolFormat::Grammar(CustomToolFormatGrammar {
        grammar: CustomToolFormatGrammarInner {
          definition: definition.into(),
          syntax,
        },
        kind: "grammar".to_string(),
      });
      tool.custom.format = Some(format);
    }
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FunctionTool {
  #[serde(rename = "function")]
  function: FunctionToolFunction,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FunctionToolFunction {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description")]
  description: Option<String>,

  #[serde(rename = "parameters")]
  parameters: Option<Schema>,

  #[serde(rename = "strict")]
  strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CustomTool {
  #[serde(rename = "custom")]
  custom: CustomToolInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CustomToolInner {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description")]
  description: Option<String>,

  #[serde(rename = "format")]
  format: Option<CustomToolFormat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum CustomToolFormat {
  Text(CustomToolFormatText),
  Grammar(CustomToolFormatGrammar),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CustomToolFormatText {
  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CustomToolFormatGrammar {
  #[serde(rename = "grammar")]
  grammar: CustomToolFormatGrammarInner,

  #[serde(rename = "type")]
  kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CustomToolFormatGrammarInner {
  #[serde(rename = "definition")]
  definition: String,

  #[serde(rename = "syntax")]
  syntax: CustomToolGrammarSyntax,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum CustomToolGrammarSyntax {
  #[serde(rename = "lark")]
  Lark,

  #[serde(rename = "regex")]
  Regex,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum VerbosityParam {
  #[serde(rename = "low")]
  Low,

  #[serde(rename = "medium")]
  Medium,

  #[serde(rename = "high")]
  High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchOptionsParam {
  #[serde(
    rename = "search_context_size",
    skip_serializing_if = "Option::is_none"
  )]
  search_context_size: Option<String>,

  #[serde(
    rename = "user_location",
    skip_serializing_if = "Option::is_none"
  )]
  user_location: Option<UserLocation>,
}

impl WebSearchOptionsParam {
  pub fn new() -> Self {
    WebSearchOptionsParam {
      search_context_size: None,
      user_location: None,
    }
  }
  pub fn with_search_context_size(mut self, search_context_size: impl Into<String>) -> Self {
    self.search_context_size = Some(search_context_size.into());
    self
  }

  pub fn with_user_city(mut self, city: impl Into<String>) -> Self {
    let mut approximate = self.user_location.take().unwrap_or(UserLocation::new());
    approximate.approximate.city = Some(city.into());
    self.user_location = Some(approximate);
    self
  }

  pub fn with_user_country(mut self, country: impl Into<String>) -> Self {
    let mut approximate = self.user_location.take().unwrap_or(UserLocation::new());
    approximate.approximate.country = Some(country.into());
    self.user_location = Some(approximate);
    self
  }

  pub fn with_user_region(mut self, region: impl Into<String>) -> Self {
    let mut approximate = self.user_location.take().unwrap_or(UserLocation::new());
    approximate.approximate.region = Some(region.into());
    self.user_location = Some(approximate);
    self
  }

  pub fn with_user_timezone(mut self, timezone: impl Into<String>) -> Self {
    let mut approximate = self.user_location.take().unwrap_or(UserLocation::new());
    approximate.approximate.timezone = Some(timezone.into());
    self.user_location = Some(approximate);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserLocation {
  #[serde(rename = "approximate")]
  approximate: Approximate,

  #[serde(rename = "type")]
  kind: String,
}

impl UserLocation {
  fn new() -> Self {
    UserLocation {
      approximate: Approximate {
        city: None,
        country: None,
        region: None,
        timezone: None,
      },
      kind: "approximate".to_string(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Approximate {
  #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
  city: Option<String>,

  #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
  country: Option<String>,

  #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
  region: Option<String>,

  #[serde(
    rename = "timezone",
    skip_serializing_if = "Option::is_none"
  )]
  timezone: Option<String>,
}

pub type LogitBiasParam = HashMap<String, i32>;
pub type MetadataParam = HashMap<String, String>;
