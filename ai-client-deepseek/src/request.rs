use ai_client_common::{
  common::MessageRole,
  request::{ResponseFormatParam, StopParam, StreamOptionsParam, ToolChoicesParam},
};
use schemars::Schema;
use serde::{Deserialize, Serialize};

use crate::model::DeepSeekChatModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSeekChatReq {
  #[serde(rename = "messages", skip_serializing_if = "Vec::is_empty")]
  messages: Vec<MessageParam>,

  #[serde(rename = "model")]
  model: DeepSeekChatModel,

  #[serde(rename = "thinking")]
  thinking: Option<ThinkingParam>,

  #[serde(rename = "frequency_penalty", skip_serializing_if = "Option::is_none")]
  frequency_penalty: Option<f32>,

  #[serde(rename = "max_tokens", skip_serializing_if = "Option::is_none")]
  max_tokens: Option<i32>,

  #[serde(rename = "presence_penalty", skip_serializing_if = "Option::is_none")]
  presence_penalty: Option<f32>,

  #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
  response_format: Option<ResponseFormatParam>,

  #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
  stop: Option<StopParam>,

  #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
  stream: Option<bool>,

  #[serde(rename = "stream_options", skip_serializing_if = "Option::is_none")]
  stream_options: Option<StreamOptionsParam>,

  #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,

  #[serde(rename = "top_p", skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,

  #[serde(rename = "tools", skip_serializing_if = "Vec::is_empty")]
  tools: Vec<ToolParam>,

  #[serde(rename = "tool_choice", skip_serializing_if = "Option::is_none")]
  tool_choice: Option<ToolChoicesParam>,

  #[serde(rename = "logprobs", skip_serializing_if = "Option::is_none")]
  log_prob: Option<bool>,

  #[serde(rename = "top_logprobs", skip_serializing_if = "Option::is_none")]
  top_log_prob: Option<i32>,
}

impl DeepSeekChatReq {
  pub fn new(model: impl Into<DeepSeekChatModel>, messages: Vec<MessageParam>) -> Self {
    DeepSeekChatReq {
      messages,
      model: model.into(),
      thinking: None,
      frequency_penalty: None,
      max_tokens: None,
      presence_penalty: None,
      response_format: None,
      stop: None,
      stream: None,
      stream_options: None,
      temperature: None,
      top_p: None,
      tools: Vec::new(),
      tool_choice: None,
      log_prob: None,
      top_log_prob: None,
    }
  }

  pub fn with_model(mut self, model: impl Into<DeepSeekChatModel>) -> Self {
    self.model = model.into();
    self
  }

  pub fn with_messages(mut self, messages: Vec<MessageParam>) -> Self {
    self.messages = messages;
    self
  }

  pub fn with_thinking(mut self, thinking: ThinkingParam) -> Self {
    self.thinking = Some(thinking);
    self
  }

  pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
    self.frequency_penalty = Some(frequency_penalty);
    self
  }

  pub fn with_max_tokens(mut self, max_tokens: i32) -> Self {
    self.max_tokens = Some(max_tokens);
    self
  }

  pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
    self.presence_penalty = Some(presence_penalty);
    self
  }

  pub fn with_response_format(mut self, response_format: ResponseFormatParam) -> Self {
    self.response_format = Some(response_format);
    self
  }

  pub fn with_stop(mut self, stop: StopParam) -> Self {
    self.stop = Some(stop);
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

  pub fn with_top_p(mut self, top_p: f32) -> Self {
    self.top_p = Some(top_p);
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

  pub fn with_log_prob(mut self, log_prob: bool) -> Self {
    self.log_prob = Some(log_prob);
    self
  }

  pub fn with_top_log_prob(mut self, top_log_prob: i32) -> Self {
    self.top_log_prob = Some(top_log_prob);
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
      name: None,
    };
    Self::System(message)
  }

  pub fn system_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let Self::System(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn new_user_message(content: impl Into<String>) -> Self {
    let message = UserMessage {
      content: content.into(),
      role: MessageRole::User,
      name: None,
    };
    Self::User(message)
  }

  pub fn user_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let Self::User(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn new_assistant_message(content: impl Into<String>) -> Self {
    let message = AssistantMessage {
      content: content.into(),
      role: MessageRole::Assistant,
      name: None,
      prefix: None,
      reasoning_content: None,
    };
    Self::Assistant(message)
  }

  pub fn assistant_message_with_name(mut self, name: impl Into<String>) -> Self {
    if let Self::Assistant(message) = &mut self {
      message.name = Some(name.into());
    }
    self
  }

  pub fn assistant_message_with_prefix(mut self, prefix: bool) -> Self {
    if let Self::Assistant(message) = &mut self {
      message.prefix = Some(prefix);
    }
    self
  }

  pub fn assistant_message_with_reasoning_content(
    mut self,
    reasoning_content: impl Into<String>,
  ) -> Self {
    if let Self::Assistant(message) = &mut self {
      message.reasoning_content = Some(reasoning_content.into());
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

  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct UserMessage {
  #[serde(rename = "content")]
  content: String,

  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssistantMessage {
  #[serde(rename = "content")]
  content: String,

  #[serde(rename = "role")]
  role: MessageRole,

  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
  prefix: Option<bool>,

  #[serde(rename = "reasoning_content", skip_serializing_if = "Option::is_none")]
  reasoning_content: Option<String>,
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
pub struct ThinkingParam {
  #[serde(rename = "type")]
  kind: String,
}

impl ThinkingParam {
  pub fn new_with_thinking(thinking: bool) -> Self {
    let kind = if thinking { "enable" } else { "disable" };
    ThinkingParam {
      kind: kind.to_string(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolParam {
  #[serde(rename = "type")]
  kind: String,

  #[serde(rename = "function")]
  function: ToolFunction,
}

impl ToolParam {
  pub fn new_function_tool(name: impl Into<String>) -> Self {
    ToolParam {
      kind: "function".to_string(),
      function: ToolFunction {
        name: name.into(),
        description: None,
        parameters: None,
        strict: None,
      },
    }
  }

  pub fn function_with_description(mut self, description: impl Into<String>) -> Self {
    self.function.description = Some(description.into());
    self
  }

  pub fn function_with_parameters(mut self, parameters: Schema) -> Self {
    self.function.parameters = Some(parameters);
    self
  }

  pub fn function_with_strict(mut self, strict: bool) -> Self {
    self.function.strict = Some(strict);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ToolFunction {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
  parameters: Option<Schema>,

  #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
  strict: Option<bool>,
}
