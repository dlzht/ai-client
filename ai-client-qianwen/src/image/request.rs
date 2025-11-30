use serde::{Deserialize, Serialize};

use crate::image::model::QianWenImageModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QianWenImageReq {
  #[serde(rename = "model")]
  model: QianWenImageModel,

  #[serde(rename = "input")]
  input: InputParam,

  #[serde(rename = "parameters")]
  parameters: Option<ParameterParam>,
}

impl QianWenImageReq {
  pub fn new(model: impl Into<QianWenImageModel>, text: impl Into<String>) -> Self {
    let input = InputParam {
      messages: vec![InputMessage {
        role: "user".to_string(),
        content: vec![MessageContent { text: text.into() }],
      }],
    };
    QianWenImageReq {
      model: model.into(),
      input,
      parameters: None,
    }
  }

  pub fn with_parameters(mut self, parameters: ParameterParam) -> Self {
    self.parameters = Some(parameters);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct InputParam {
  #[serde(rename = "messages")]
  messages: Vec<InputMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct InputMessage {
  #[serde(rename = "role")]
  role: String,

  #[serde(rename = "content")]
  content: Vec<MessageContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MessageContent {
  #[serde(rename = "text")]
  text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterParam {
  #[serde(rename = "negative_prompt", skip_serializing_if = "Option::is_none")]
  negative_prompt: Option<String>,

  #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
  size: Option<ImageSize>,

  #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
  n: Option<i32>,

  #[serde(rename = "prompt_extend", skip_serializing_if = "Option::is_none")]
  prompt_extend: Option<bool>,

  #[serde(rename = "watermark", skip_serializing_if = "Option::is_none")]
  watermark: Option<bool>,

  #[serde(rename = "seed", skip_serializing_if = "Option::is_none")]
  seed: Option<i32>,
}

impl ParameterParam {
  pub fn new() -> Self {
    ParameterParam {
      negative_prompt: None,
      size: None,
      n: None,
      prompt_extend: None,
      watermark: None,
      seed: None,
    }
  }

  pub fn with_negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
    self.negative_prompt = Some(negative_prompt.into());
    self
  }

  pub fn with_size(mut self, size: ImageSize) -> Self {
    self.size = Some(size);
    self
  }

  pub fn with_n(mut self, n: i32) -> Self {
    self.n = Some(n);
    self
  }

  pub fn with_prompt_extend(mut self, prompt_extend: bool) -> Self {
    self.prompt_extend = Some(prompt_extend);
    self
  }

  pub fn with_watermark(mut self, watermark: bool) -> Self {
    self.watermark = Some(watermark);
    self
  }

  pub fn with_seed(mut self, seed: i32) -> Self {
    self.seed = Some(seed);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageSize {
  #[serde(rename = "1664*928")]
  W1664H928,

  #[serde(rename = "1472*1140")]
  W1472H1140,

  #[serde(rename = "1328*1328")]
  W1328H1328,

  #[serde(rename = "1140*1472")]
  W1140H1472,

  #[serde(rename = "928*1664")]
  W928H1664,

  #[serde(untagged)]
  Other(String),
}

#[cfg(test)]
mod test {
  use crate::image::request::QianWenImageReq;

  #[test]
  fn test_serialize_req() {
    let json0 = "{\"model\":\"qwen-image-plus\",\"input\":{\"messages\":[{\"role\":\"user\",\"content\":[{\"text\":\"一副典雅庄重的对联悬挂于厅堂之中，房间是个安静古典的中式布置，桌子上放着一些青花瓷，对联上左书“义本生知人机同道善思新”，右书“通云赋智乾坤启数高志远”，横批“智启通义”，字体飘逸，在中间挂着一幅中国风的画作，内容是岳阳楼。\"}]}]},\"parameters\":{\"negative_prompt\":\"\",\"prompt_extend\":true,\"watermark\":false,\"size\":\"1328*1328\"}}";
    assert!(serde_json::from_str::<QianWenImageReq>(json0).is_ok());
  }
}
