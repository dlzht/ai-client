use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopParam {
  Text(String),
  Array(Vec<String>),
}

impl StopParam {
  pub fn new_with_text(text: impl Into<String>) -> Self {
    StopParam::Text(text.into())
  }

  pub fn new_with_array(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {
    let texts = texts.into_iter().map(|s| s.into()).collect();
    StopParam::Array(texts)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamOptionsParam {
  #[serde(rename = "include_usage")]
  include_usage: bool,
}

impl StreamOptionsParam {
  pub fn new_with_usage(include_usage: bool) -> Self {
    StreamOptionsParam { include_usage }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormatParam {
  #[serde(rename = "type")]
  kind: String,
}

impl ResponseFormatParam {
  pub fn new_text_format() -> Self {
    ResponseFormatParam {
      kind: "text".to_string(),
    }
  }

  pub fn new_json_format() -> Self {
    ResponseFormatParam {
      kind: "json_object".to_string(),
    }
  }
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
  pub fn new_with_force_function(name: impl Into<String>) -> Self {
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
