use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum QianWenImageModel {
  #[serde(rename = "qwen-image-plus")]
  QwenImagePlus,
  
  #[serde(rename = "qwen-image")]
  QwenImage,
  
  #[serde(untagged)]
  Other(String)
}

impl QianWenImageModel {
  pub fn to_str(&self) -> &str {
    match self {
      QianWenImageModel::QwenImagePlus => "qwen-image-plus",
      QianWenImageModel::QwenImage => "qwen-image",
      QianWenImageModel::Other(s) => s.as_str(),
    }
  }
  
  pub fn from_str(s: &str) -> QianWenImageModel {
    match s {
      "qwen-image-plus" => QianWenImageModel::QwenImagePlus,
      "qwen-image" => QianWenImageModel::QwenImage,
      _ => QianWenImageModel::Other(s.to_string()),
    }
  }
}

impl From<&str> for QianWenImageModel {
  fn from(s: &str) -> Self {
    QianWenImageModel::from_str(s)
  }
}