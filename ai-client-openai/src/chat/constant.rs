use crate::chat::model::OpenAiChatModel;

pub const OPEN_AI_DEFAULT_CHAT_ENDPOINT: &'static str =
  "https://api.openai.com/v1/chat/completions";
pub const OPEN_AI_DEFAULT_CHAT_MODEL: OpenAiChatModel = OpenAiChatModel::Gpt4OMini;
