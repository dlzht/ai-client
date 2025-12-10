use std::collections::HashMap;

use ai_client_common::common::EmptyType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
  common::{
    AspectRatio, ComputerUserEnvironment, Content, DynamicRetrievalMode, FunctionBehavior,
    FunctionCallingMode, HarmBlockThreshold, HarmCategory, ImageSize, MediaResolution, Modality,
    ThinkingLevel,
  },
  model::GeminiModel,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiReq {
  #[serde(skip)]
  pub(crate) model: Option<GeminiModel>,

  #[serde(rename = "contents")]
  contents: Vec<Content>,

  #[serde(rename = "tools", skip_serializing_if = "Vec::is_empty", default)]
  tools: Vec<Tool>,

  #[serde(rename = "toolConfig", skip_serializing_if = "Option::is_none")]
  tool_config: Option<ToolConfig>,

  #[serde(
    rename = "safetySettings",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  safety_settings: Vec<SafetySetting>,

  #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
  system_instruction: Option<Content>,

  #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
  generation_config: Option<GenerationConfig>,

  #[serde(rename = "cacheContent", skip_serializing_if = "Option::is_none")]
  cache_content: Option<String>,
}

impl GeminiReq {
  pub fn new(contents: Vec<Content>) -> Self {
    GeminiReq {
      model: None,
      contents,
      tools: Vec::new(),
      tool_config: None,
      safety_settings: Vec::new(),
      system_instruction: None,
      generation_config: None,
      cache_content: None,
    }
  }

  pub fn with_model(mut self, model: GeminiModel) -> Self {
    self.model = Some(model);
    self
  }

  pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
    self.tools = tools;
    self
  }

  pub fn with_tool_config(mut self, tool_config: ToolConfig) -> Self {
    self.tool_config = Some(tool_config);
    self
  }

  pub fn with_safety_settings(mut self, safety_settings: Vec<SafetySetting>) -> Self {
    self.safety_settings = safety_settings;
    self
  }

  pub fn with_system_instruction(mut self, system_instruction: Content) -> Self {
    self.system_instruction = Some(system_instruction);
    self
  }

  pub fn with_generation_config(mut self, generation_config: GenerationConfig) -> Self {
    self.generation_config = Some(generation_config);
    self
  }

  pub fn with_cache_content(mut self, cache_content: String) -> Self {
    self.cache_content = Some(cache_content);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
  #[serde(
    rename = "functionDeclarations",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  function_declarations: Vec<FunctionDeclaration>,

  #[serde(
    rename = "googleSearchRetrieval",
    skip_serializing_if = "Option::is_none"
  )]
  google_search_retrieval: Option<GoogleSearchRetrieval>,

  #[serde(rename = "codeExecution", skip_serializing_if = "Option::is_none")]
  code_execution: Option<EmptyType>,

  #[serde(rename = "googleSearch", skip_serializing_if = "Option::is_none")]
  google_search: Option<GoogleSearch>,

  #[serde(rename = "computerUse", skip_serializing_if = "Option::is_none")]
  computer_user: Option<ComputerUse>,

  #[serde(rename = "urlContext", skip_serializing_if = "Option::is_none")]
  url_context: Option<EmptyType>,

  #[serde(rename = "fileSearch", skip_serializing_if = "Option::is_none")]
  file_search: Option<FileSearch>,

  #[serde(rename = "googleMaps", skip_serializing_if = "Option::is_none")]
  google_maps: Option<GoogleMaps>,
}

impl Tool {
  pub fn new() -> Self {
    Tool {
      function_declarations: vec![],
      google_search_retrieval: None,
      code_execution: None,
      google_search: None,
      computer_user: None,
      url_context: None,
      file_search: None,
      google_maps: None,
    }
  }

  pub fn with_function_declarations(mut self, functions: Vec<FunctionDeclaration>) -> Self {
    self.function_declarations = functions;
    self
  }

  pub fn with_google_search_retrieval(
    mut self,
    google_search_retrieval: GoogleSearchRetrieval,
  ) -> Self {
    self.google_search_retrieval = Some(google_search_retrieval);
    self
  }

  pub fn with_code_execution(mut self) -> Self {
    self.code_execution = Some(EmptyType {});
    self
  }

  pub fn with_google_search(mut self, google_search: GoogleSearch) -> Self {
    self.google_search = Some(google_search);
    self
  }

  pub fn with_computer_user(mut self, computer_user: ComputerUse) -> Self {
    self.computer_user = Some(computer_user);
    self
  }

  pub fn with_url_context(mut self) -> Self {
    self.url_context = Some(EmptyType {});
    self
  }

  pub fn with_file_search(mut self, file_search: FileSearch) -> Self {
    self.file_search = Some(file_search);
    self
  }

  pub fn with_google_maps(mut self, google_maps: GoogleMaps) -> Self {
    self.google_maps = Some(google_maps);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDeclaration {
  #[serde(rename = "name")]
  name: String,

  #[serde(rename = "description")]
  description: String,

  #[serde(rename = "behavior", skip_serializing_if = "Option::is_none")]
  behavior: Option<FunctionBehavior>,

  #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
  parameters: Option<Schema>,

  #[serde(
    rename = "parametersJsonSchema",
    skip_serializing_if = "Option::is_none"
  )]
  parameters_json_schema: Option<Value>,

  #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
  response: Option<Schema>,

  #[serde(rename = "responseJsonSchema", skip_serializing_if = "Option::is_none")]
  response_json_schema: Option<Value>,
}

impl FunctionDeclaration {
  pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
    FunctionDeclaration {
      name: name.into(),
      description: description.into(),
      behavior: None,
      parameters: None,
      parameters_json_schema: None,
      response: None,
      response_json_schema: None,
    }
  }

  pub fn with_behavior(mut self, behavior: FunctionBehavior) -> Self {
    self.behavior = Some(behavior);
    self
  }

  pub fn with_parameters(mut self, parameters: Schema) -> Self {
    self.parameters = Some(parameters);
    self
  }

  pub fn with_parameters_json_schema(mut self, parameters_json_schema: Value) -> Self {
    self.parameters_json_schema = Some(parameters_json_schema);
    self
  }

  pub fn with_response(mut self, response: Schema) -> Self {
    self.response = Some(response);
    self
  }

  pub fn with_response_json_schema(mut self, response_json_schema: Value) -> Self {
    self.response_json_schema = Some(response_json_schema);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleSearchRetrieval {
  #[serde(rename = "dynamicRetrievalConfig")]
  dynamic_retrieval_config: DynamicRetrievalConfig,
}

impl GoogleSearchRetrieval {
  pub fn new(mode: DynamicRetrievalMode) -> Self {
    GoogleSearchRetrieval {
      dynamic_retrieval_config: DynamicRetrievalConfig {
        mode,
        dynamic_threshold: None,
      },
    }
  }

  pub fn with_dynamic_threshold(mut self, dynamic_threshold: f32) -> Self {
    self.dynamic_retrieval_config.dynamic_threshold = Some(dynamic_threshold);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DynamicRetrievalConfig {
  #[serde(rename = "mode")]
  mode: DynamicRetrievalMode,

  #[serde(rename = "dynamicThreshold", skip_serializing_if = "Option::is_none")]
  dynamic_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputerUse {
  #[serde(rename = "environment")]
  environment: ComputerUserEnvironment,

  #[serde(
    rename = "excludedPredefinedFunctions",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  excluded_predefined_functions: Vec<String>,
}

impl ComputerUse {
  pub fn new(environment: ComputerUserEnvironment) -> Self {
    ComputerUse {
      environment,
      excluded_predefined_functions: Vec::new(),
    }
  }

  pub fn with_excluded_predefined_functions(mut self, functions: Vec<String>) -> Self {
    self.excluded_predefined_functions = functions;
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSearch {
  #[serde(
    rename = "fileSearchStoreNames",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  file_search_store_names: Vec<String>,

  #[serde(rename = "metadataFilter", skip_serializing_if = "Option::is_none")]
  metadata_filter: Option<String>,

  #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
  top_k: Option<i32>,
}

impl FileSearch {
  pub fn new(file_search_store_names: Vec<String>) -> Self {
    FileSearch {
      file_search_store_names,
      metadata_filter: None,
      top_k: None,
    }
  }

  pub fn with_metadata_filter(mut self, metadata_filter: String) -> Self {
    self.metadata_filter = Some(metadata_filter);
    self
  }

  pub fn with_top_k(mut self, top_k: i32) -> Self {
    self.top_k = Some(top_k);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleSearch {
  #[serde(rename = "timeRangeFilter", skip_serializing_if = "Option::is_none")]
  time_range_filter: Option<Interval>,
}

impl GoogleSearch {
  pub fn new() -> Self {
    GoogleSearch {
      time_range_filter: None,
    }
  }

  pub fn with_strat_time_filter(mut self, start_time: String) -> Self {
    let mut filter = self.time_range_filter.take().unwrap_or(Interval {
      start_time: None,
      end_time: None,
    });
    filter.start_time = Some(start_time);
    self.time_range_filter = Some(filter);
    self
  }

  pub fn with_end_time_filter(mut self, end_time: String) -> Self {
    let mut filter = self.time_range_filter.take().unwrap_or(Interval {
      start_time: None,
      end_time: None,
    });
    filter.end_time = Some(end_time);
    self.time_range_filter = Some(filter);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleMaps {
  #[serde(rename = "enableWidget", skip_serializing_if = "Option::is_none")]
  enable_widget: Option<bool>,
}

impl GoogleMaps {
  pub fn new() -> Self {
    GoogleMaps {
      enable_widget: None,
    }
  }

  pub fn with_enable_widget(mut self, enable_widget: bool) -> Self {
    self.enable_widget = Some(enable_widget);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolConfig {
  #[serde(
    rename = "functionCallingConfig",
    skip_serializing_if = "Option::is_none"
  )]
  function_calling_config: Option<FunctionCallingConfig>,
}

impl ToolConfig {
  pub fn new() -> Self {
    ToolConfig {
      function_calling_config: None,
    }
  }

  pub fn with_function_calling_mode(mut self, function_calling_mode: FunctionCallingMode) -> Self {
    let mut function_calling_config =
      self
        .function_calling_config
        .take()
        .unwrap_or(FunctionCallingConfig {
          mode: None,
          allowed_function_names: Vec::new(),
        });
    function_calling_config.mode = Some(function_calling_mode);
    self.function_calling_config = Some(function_calling_config);
    self
  }

  pub fn with_function_calling_allowed_functions(
    mut self,
    function_calling_allowed_functions: Vec<String>,
  ) -> Self {
    let mut function_calling_config =
      self
        .function_calling_config
        .take()
        .unwrap_or(FunctionCallingConfig {
          mode: None,
          allowed_function_names: Vec::new(),
        });
    function_calling_config.allowed_function_names = function_calling_allowed_functions;
    self.function_calling_config = Some(function_calling_config);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FunctionCallingConfig {
  #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
  mode: Option<FunctionCallingMode>,

  #[serde(
    rename = "allowedFunctionNames",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  allowed_function_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetrievalConfig {
  #[serde(rename = "latLng")]
  location: Location,

  #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
  language_code: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SafetySetting {
  #[serde(rename = "category")]
  category: HarmCategory,

  #[serde(rename = "threshold")]
  threshold: HarmBlockThreshold,
}

impl SafetySetting {
  pub fn new(category: HarmCategory, threshold: HarmBlockThreshold) -> Self {
    SafetySetting {
      category,
      threshold,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationConfig {
  #[serde(
    rename = "stopSequences",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  stop_sequences: Vec<String>,

  #[serde(rename = "responseMimeType", skip_serializing_if = "Option::is_none")]
  response_mime_type: Option<String>,

  #[serde(rename = "responseSchema", skip_serializing_if = "Option::is_none")]
  response_schema: Option<Schema>,

  #[serde(
    rename = "_responseJsonSchema",
    skip_serializing_if = "Option::is_none"
  )]
  _response_json_schema: Option<Value>,

  #[serde(rename = "responseJsonSchema", skip_serializing_if = "Option::is_none")]
  response_json_schema: Option<Value>,

  #[serde(
    rename = "responseModalities",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  response_modalities: Vec<Modality>,

  #[serde(rename = "candidateCount", skip_serializing_if = "Option::is_none")]
  candidate_count: Option<i32>,

  #[serde(rename = "maxOutputTokens", skip_serializing_if = "Option::is_none")]
  max_output_tokens: Option<i32>,

  #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
  temperature: Option<f32>,

  #[serde(rename = "topP", skip_serializing_if = "Option::is_none")]
  top_p: Option<f32>,

  #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
  top_k: Option<i32>,

  #[serde(rename = "seed", skip_serializing_if = "Option::is_none")]
  seed: Option<i32>,

  #[serde(rename = "presencePenalty", skip_serializing_if = "Option::is_none")]
  presence_penalty: Option<f32>,

  #[serde(rename = "frequencyPenalty", skip_serializing_if = "Option::is_none")]
  frequency_penalty: Option<f32>,

  #[serde(rename = "responseLogprobs", skip_serializing_if = "Option::is_none")]
  response_log_prob: Option<bool>,

  #[serde(rename = "logprobs", skip_serializing_if = "Option::is_none")]
  log_prob: Option<i32>,

  #[serde(
    rename = "enableEnhancedCivicAnswers",
    skip_serializing_if = "Option::is_none"
  )]
  enable_enhanced_civic_answers: Option<bool>,

  #[serde(rename = "speechConfig", skip_serializing_if = "Option::is_none")]
  speech_config: Option<SpeechConfig>,

  #[serde(rename = "thinkingConfig", skip_serializing_if = "Option::is_none")]
  thinking_config: Option<ThinkingConfig>,

  #[serde(rename = "imageConfig", skip_serializing_if = "Option::is_none")]
  image_config: Option<ImageConfig>,

  #[serde(rename = "mediaResolution", skip_serializing_if = "Option::is_none")]
  media_resolution: Option<MediaResolution>,
}

impl GenerationConfig {
  pub fn new() -> Self {
    GenerationConfig {
      stop_sequences: Vec::new(),
      response_mime_type: None,
      response_schema: None,
      _response_json_schema: None,
      response_json_schema: None,
      response_modalities: Vec::new(),
      candidate_count: None,
      max_output_tokens: None,
      temperature: None,
      top_p: None,
      top_k: None,
      seed: None,
      presence_penalty: None,
      frequency_penalty: None,
      response_log_prob: None,
      log_prob: None,
      enable_enhanced_civic_answers: None,
      speech_config: None,
      thinking_config: None,
      image_config: None,
      media_resolution: None,
    }
  }

  pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
    self.stop_sequences = stop_sequences;
    self
  }

  pub fn with_response_mime_type(mut self, response_mime_type: String) -> Self {
    self.response_mime_type = Some(response_mime_type);
    self
  }

  pub fn with_response_schema(mut self, response_schema: Schema) -> Self {
    self.response_schema = Some(response_schema);
    self
  }

  pub fn with_response_json_schema(mut self, response_json_schema: Value) -> Self {
    self._response_json_schema = Some(response_json_schema);
    self
  }

  pub fn with_underline_response_json_schema(mut self, response_json_schema: Value) -> Self {
    self._response_json_schema = Some(response_json_schema);
    self
  }

  pub fn with_response_modalities(mut self, response_modalities: Vec<Modality>) -> Self {
    self.response_modalities = response_modalities;
    self
  }

  pub fn with_candidate_count(mut self, candidate_count: i32) -> Self {
    self.candidate_count = Some(candidate_count);
    self
  }

  pub fn with_max_output_tokens(mut self, max_output_tokens: i32) -> Self {
    self.max_output_tokens = Some(max_output_tokens);
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

  pub fn with_seed(mut self, seed: i32) -> Self {
    self.seed = Some(seed);
    self
  }

  pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
    self.presence_penalty = Some(presence_penalty);
    self
  }

  pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
    self.frequency_penalty = Some(frequency_penalty);
    self
  }

  pub fn with_response_log_prob(mut self, response_log_prob: bool) -> Self {
    self.response_log_prob = Some(response_log_prob);
    self
  }

  pub fn with_log_prob(mut self, log_prob: i32) -> Self {
    self.log_prob = Some(log_prob);
    self
  }

  pub fn with_enable_enhanced_civic_answers(mut self, enable_enhanced_civic_answers: bool) -> Self {
    self.enable_enhanced_civic_answers = Some(enable_enhanced_civic_answers);
    self
  }

  pub fn with_speech_config(mut self, speech_config: SpeechConfig) -> Self {
    self.speech_config = Some(speech_config);
    self
  }

  pub fn with_thinking_config(mut self, thinking_config: ThinkingConfig) -> Self {
    self.thinking_config = Some(thinking_config);
    self
  }

  pub fn with_image_config(mut self, image_config: ImageConfig) -> Self {
    self.image_config = Some(image_config);
    self
  }

  pub fn with_media_resolution(mut self, media_resolution: MediaResolution) -> Self {
    self.media_resolution = Some(media_resolution);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
  #[serde(rename = "kind")]
  kind: SchemaType,

  #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
  format: Option<String>,

  #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[serde(rename = "nullable", skip_serializing_if = "Option::is_none")]
  nullable: Option<bool>,

  #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty", default)]
  enum_direction: Vec<String>,

  #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
  max_items: Option<String>,

  #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
  min_items: Option<String>,

  #[serde(
    rename = "properties",
    skip_serializing_if = "HashMap::is_empty",
    default
  )]
  properties: HashMap<String, Box<Schema>>,

  #[serde(rename = "required", skip_serializing_if = "Vec::is_empty", default)]
  required: Vec<String>,

  #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
  min_properties: Option<String>,

  #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
  max_properties: Option<String>,

  #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
  min_length: Option<String>,

  #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
  max_length: Option<String>,

  #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
  pattern: Option<String>,

  #[serde(rename = "example", skip_serializing_if = "Option::is_none")]
  example_value: Option<Value>,

  #[serde(rename = "allOf", skip_serializing_if = "Vec::is_empty", default)]
  any_of: Vec<Schema>,

  #[serde(
    rename = "propertyOrdering",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  property_ordering: Vec<String>,

  #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
  default_value: Option<Value>,

  #[serde(rename = "items", skip_serializing_if = "Vec::is_empty", default)]
  items: Vec<Schema>,

  #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
  minimum: Option<f32>,

  #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
  maximum: Option<f32>,
}

impl Schema {
  pub fn new(kind: SchemaType) -> Self {
    Self {
      kind,
      format: None,
      title: None,
      description: None,
      nullable: None,
      enum_direction: Vec::new(),
      max_items: None,
      min_items: None,
      properties: HashMap::new(),
      required: Vec::new(),
      min_properties: None,
      max_properties: None,
      min_length: None,
      max_length: None,
      pattern: None,
      example_value: None,
      any_of: Vec::new(),
      property_ordering: Vec::new(),
      default_value: None,
      items: Vec::new(),
      minimum: None,
      maximum: None,
    }
  }

  pub fn with_format(mut self, format: impl Into<String>) -> Self {
    self.format = Some(format.into());
    self
  }

  pub fn with_title(mut self, title: impl Into<String>) -> Self {
    self.title = Some(title.into());
    self
  }

  pub fn with_description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_nullable(mut self, nullable: bool) -> Self {
    self.nullable = Some(nullable);
    self
  }

  pub fn with_enum(mut self, enum_direction: Vec<String>) -> Self {
    self.enum_direction = enum_direction;
    self
  }

  pub fn with_min_items(mut self, min_items: i64) -> Self {
    self.min_items = Some(min_items.to_string());
    self
  }

  pub fn with_max_items(mut self, max_items: i64) -> Self {
    self.max_items = Some(max_items.to_string());
    self
  }

  pub fn with_min_properties(mut self, min_properties: i64) -> Self {
    self.min_properties = Some(min_properties.to_string());
    self
  }

  pub fn with_max_properties(mut self, max_properties: i64) -> Self {
    self.max_properties = Some(max_properties.to_string());
    self
  }

  pub fn with_min_length(mut self, min_length: i64) -> Self {
    self.min_length = Some(min_length.to_string());
    self
  }

  pub fn with_max_length(mut self, max_length: i64) -> Self {
    self.max_length = Some(max_length.to_string());
    self
  }

  pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
    self.pattern = Some(pattern.into());
    self
  }

  pub fn with_example(mut self, example_value: Value) -> Self {
    self.example_value = Some(example_value);
    self
  }

  pub fn with_any_of(mut self, any_of: Vec<Schema>) -> Self {
    self.any_of = any_of;
    self
  }

  pub fn with_property_ordering(mut self, property_ordering: Vec<String>) -> Self {
    self.property_ordering = property_ordering;
    self
  }

  pub fn with_default(mut self, default_value: Value) -> Self {
    self.default_value = Some(default_value);
    self
  }

  pub fn with_items(mut self, items: Vec<Schema>) -> Self {
    self.items = items;
    self
  }

  pub fn with_minimum(mut self, minimum: f32) -> Self {
    self.minimum = Some(minimum);
    self
  }

  pub fn with_maximum(mut self, maximum: f32) -> Self {
    self.maximum = Some(maximum);
    self
  }

  pub fn with_required(mut self, required: Vec<String>) -> Self {
    self.required = required;
    self
  }

  pub fn with_property(mut self, key: impl Into<String>, value: Schema) -> Self {
    self.properties.insert(key.into(), Box::new(value));
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SchemaType {
  #[serde(rename = "TYPE_UNSPECIFIED")]
  TypeUnspecified,

  #[serde(rename = "STRING")]
  String,

  #[serde(rename = "NUMBER")]
  Number,

  #[serde(rename = "INTEGER")]
  Integer,

  #[serde(rename = "BOOLEAN")]
  Boolean,

  #[serde(rename = "ARRAY")]
  Array,

  #[serde(rename = "OBJECT")]
  Object,

  #[serde(rename = "NULL")]
  Null,

  #[serde(untagged)]
  Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Interval {
  #[serde(rename = "startTime")]
  start_time: Option<String>,

  #[serde(rename = "endTime")]
  end_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
  #[serde(rename = "latitude")]
  latitude: f64,

  #[serde(rename = "longitude")]
  longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeechConfig {
  #[serde(rename = "speaker_voice")]
  voice_config: VoiceConfig,

  #[serde(
    rename = "multi_speaker_voice_config",
    skip_serializing_if = "Option::is_none"
  )]
  multi_speaker_voice_config: Option<MultiSpeakerVoiceConfig>,

  #[serde(rename = "language_code", skip_serializing_if = "Option::is_none")]
  language_code: Option<String>,
}

impl SpeechConfig {
  pub fn new_with_voice_name(voice_name: impl Into<String>) -> Self {
    Self {
      voice_config: VoiceConfig::Prebuild(PrebuiltVoiceConfig {
        voice_name: voice_name.into(),
      }),
      multi_speaker_voice_config: None,
      language_code: None,
    }
  }

  pub fn with_multi_speaker_voice_config(
    mut self,
    voices: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
  ) -> Self {
    let voices = voices
      .into_iter()
      .map(|(speaker, name)| SpeakerVoiceConfig {
        speaker: speaker.into(),
        voice_config: VoiceConfig::Prebuild(PrebuiltVoiceConfig {
          voice_name: name.into(),
        }),
      })
      .collect();
    self.multi_speaker_voice_config = Some(MultiSpeakerVoiceConfig {
      speaker_voice_configs: voices,
    });
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum VoiceConfig {
  #[serde(rename = "prebuiltVoiceConfig")]
  Prebuild(PrebuiltVoiceConfig),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PrebuiltVoiceConfig {
  #[serde(rename = "voice_name")]
  voice_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MultiSpeakerVoiceConfig {
  #[serde(
    rename = "speaker_voice_configs",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  speaker_voice_configs: Vec<SpeakerVoiceConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SpeakerVoiceConfig {
  #[serde(rename = "speaker")]
  speaker: String,

  #[serde(rename = "voice_config")]
  voice_config: VoiceConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThinkingConfig {
  #[serde(rename = "include_thoughts")]
  include_thoughts: bool,

  #[serde(rename = "thinking_budget")]
  thinking_budget: i32,

  #[serde(rename = "thinking_level", skip_serializing_if = "Option::is_none")]
  thinking_level: Option<ThinkingLevel>,
}

impl ThinkingConfig {
  pub fn new(include_thoughts: bool, thinking_budget: i32) -> Self {
    Self {
      include_thoughts,
      thinking_budget,
      thinking_level: None,
    }
  }

  pub fn with_thinking_level(mut self, thinking_level: ThinkingLevel) -> Self {
    self.thinking_level = Some(thinking_level);
    self
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageConfig {
  #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
  aspect_ratio: Option<AspectRatio>,

  #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
  image_size: Option<ImageSize>,
}

impl ImageConfig {
  pub fn new() -> Self {
    Self {
      aspect_ratio: None,
      image_size: None,
    }
  }

  pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
    self.aspect_ratio = Some(aspect_ratio);
    self
  }

  pub fn with_image_size(mut self, image_size: ImageSize) -> Self {
    self.image_size = Some(image_size);
    self
  }
}
