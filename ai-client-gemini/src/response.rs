use serde::{Deserialize, Serialize};

use crate::common::{
  BlockReason, Content, FinishReason, HarmCategory, HarmProbability, Modality, UrlRetrievalStatus,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiRes {
  #[serde(rename = "candidates", skip_serializing_if = "Vec::is_empty", default)]
  candidates: Vec<Candidate>,

  #[serde(rename = "promptFeedback")]
  prompt_feedback: PromptFeedback,

  #[serde(rename = "usageMetadata")]
  usage: UsageData,

  #[serde(rename = "modelVersion")]
  model_version: String,

  #[serde(rename = "requestId")]
  response_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candidate {
  #[serde(rename = "content")]
  content: Option<Content>,

  #[serde(rename = "finishReason")]
  finish_reason: Option<FinishReason>,

  #[serde(
    rename = "safetyRatings",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  safety_ratings: Vec<SafetyRating>,

  #[serde(rename = "citationMetadata", skip_serializing_if = "Option::is_none")]
  citation_metadata: Option<CitationMetadata>,

  #[serde(rename = "tokenCount", skip_serializing_if = "Option::is_none")]
  token_count: Option<i32>,

  #[serde(
    rename = "groundingAttributions",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  grounding_attributions: Vec<GroundingAttribution>,

  #[serde(rename = "groundingMetadata", skip_serializing_if = "Option::is_none")]
  grounding_metadata: Option<GroundingMetadata>,

  #[serde(rename = "avgLogprobs", skip_serializing_if = "Option::is_none")]
  avg_log_prob: Option<f32>,

  #[serde(rename = "logprobsResult", skip_serializing_if = "Option::is_none")]
  log_prob_result: Option<LogProbResult>,

  #[serde(rename = "urlContextMetadata", skip_serializing_if = "Option::is_none")]
  url_context_metadata: Option<UrlContextMetadata>,

  #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
  index: Option<i32>,

  #[serde(rename = "finishMessage", skip_serializing_if = "Option::is_none")]
  finish_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageData {
  #[serde(rename = "promptTokenCount")]
  prompt_token_count: i32,

  #[serde(
    rename = "cachedContentTokenCount",
    skip_serializing_if = "Option::is_none"
  )]
  cached_content_token_count: Option<i32>,

  #[serde(
    rename = "candidatesTokenCount",
    skip_serializing_if = "Option::is_none"
  )]
  candidates_token_count: Option<i32>,

  #[serde(
    rename = "toolUsePromptTokenCount",
    skip_serializing_if = "Option::is_none"
  )]
  tool_use_prompt_token_count: Option<i32>,

  #[serde(rename = "thoughtsTokenCount", skip_serializing_if = "Option::is_none")]
  thoughts_token_count: Option<i32>,

  #[serde(rename = "totalTokenCount", skip_serializing_if = "Option::is_none")]
  total_token_count: Option<i32>,

  #[serde(
    rename = "promptTokensDetails",
    skip_serializing_if = "Option::is_none"
  )]
  prompt_tokens_details: Option<Vec<ModalityTokenCount>>,

  #[serde(rename = "cacheTokensDetails", skip_serializing_if = "Option::is_none")]
  cache_tokens_details: Option<Vec<ModalityTokenCount>>,

  #[serde(
    rename = "candidatesTokensDetails",
    skip_serializing_if = "Option::is_none"
  )]
  candidates_tokens_details: Option<Vec<ModalityTokenCount>>,

  #[serde(
    rename = "toolUsePromptTokensDetails",
    skip_serializing_if = "Option::is_none"
  )]
  tool_use_prompt_tokens_details: Option<Vec<ModalityTokenCount>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModalityTokenCount {
  #[serde(rename = "modality")]
  modality: Modality,

  #[serde(rename = "tokenCount")]
  token_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SafetyRating {
  #[serde(rename = "category")]
  category: HarmCategory,

  #[serde(rename = "probability")]
  probability: HarmProbability,

  #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CitationMetadata {
  #[serde(
    rename = "citationSources",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  citation_sources: Vec<CitationSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CitationSource {
  #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
  start_index: Option<i32>,

  #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
  end_index: Option<i32>,

  #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
  uri: Option<String>,

  #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
  license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingAttribution {
  #[serde(rename = "sourceId", skip_serializing_if = "Option::is_none")]
  source_id: Option<AttributionSourceId>,

  #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
  content: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttributionSourceId {
  #[serde(rename = "groundingPassage")]
  GroundingPassage(GroundingPassage),

  #[serde(rename = "semanticRetrieverChunk")]
  SemanticRetrieverChunk(SemanticRetrieverChunk),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingPassage {
  #[serde(rename = "passageId", skip_serializing_if = "Option::is_none")]
  passage_id: Option<String>,

  #[serde(rename = "partIndex", skip_serializing_if = "Option::is_none")]
  part_index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemanticRetrieverChunk {
  #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
  source: Option<String>,

  #[serde(rename = "chunk", skip_serializing_if = "Option::is_none")]
  chunk: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingMetadata {
  #[serde(
    rename = "groundingChunks",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  grounding_chunks: Vec<GroundingChunk>,

  #[serde(
    rename = "groundingSupports",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  grounding_supports: Vec<GroundingSupport>,

  #[serde(
    rename = "webSearchQueries",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  web_search_queries: Vec<String>,

  #[serde(rename = "searchEntryPoint", skip_serializing_if = "Option::is_none")]
  search_entry_point: Option<SearchEntryPoint>,

  #[serde(rename = "retrievalMetadata", skip_serializing_if = "Option::is_none")]
  retrieval_metadata: Option<RetrievalMetadata>,

  #[serde(
    rename = "googleMapsWidgetContextToken",
    skip_serializing_if = "Option::is_none"
  )]
  google_maps_widget_context_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingChunk {
  #[serde(rename = "web")]
  web: GroundingChunkWeb,

  #[serde(rename = "retrievedContext", skip_serializing_if = "Option::is_none")]
  retrieved_context: Option<RetrievedContext>,

  #[serde(rename = "maps", skip_serializing_if = "Option::is_none")]
  maps: Option<GroundingChunkMaps>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingChunkWeb {
  #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
  uri: Option<String>,

  #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
  title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetrievedContext {
  #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
  uri: Option<String>,

  #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
  text: Option<String>,

  #[serde(rename = "fileSearchStore", skip_serializing_if = "Option::is_none")]
  file_search_store: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingChunkMaps {
  #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
  uri: Option<String>,

  #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[serde(rename = "placeId", skip_serializing_if = "Option::is_none")]
  place_id: Option<String>,

  #[serde(rename = "placeAnswerSources", skip_serializing_if = "Option::is_none")]
  place_answer_sources: Option<PlaceAnswerSources>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceAnswerSources {
  #[serde(
    rename = "reviewSnippets",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  review_snippets: Vec<ReviewSnippet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReviewSnippet {
  #[serde(rename = "reviewId", skip_serializing_if = "Option::is_none")]
  review_id: Option<String>,

  #[serde(rename = "googleMapsUri", skip_serializing_if = "Option::is_none")]
  google_maps_uri: Option<String>,

  #[serde(rename = "title")]
  title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroundingSupport {
  #[serde(
    rename = "groundingChunkIndices",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  grounding_chunk_indices: Vec<i32>,

  #[serde(
    rename = "confidenceScores",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  confidence_scores: Vec<f32>,

  #[serde(rename = "segment")]
  segment: Option<ContentSegment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentSegment {
  #[serde(rename = "partIndex", skip_serializing_if = "Option::is_none")]
  part_index: Option<i32>,

  #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
  start_index: Option<i32>,

  #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
  end_index: Option<i32>,

  #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
  text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEntryPoint {
  #[serde(rename = "renderedContent", skip_serializing_if = "Option::is_none")]
  rendered_content: Option<String>,

  #[serde(rename = "sdkBlob", skip_serializing_if = "Option::is_none")]
  sdk_blob: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetrievalMetadata {
  #[serde(
    rename = "googleSearchDynamicRetrievalScore",
    skip_serializing_if = "Option::is_none"
  )]
  google_search_dynamic_retrieval_score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptFeedback {
  #[serde(rename = "blockReason", skip_serializing_if = "Option::is_none")]
  block_reason: Option<BlockReason>,

  #[serde(
    rename = "safetyRatings",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbResult {
  #[serde(
    rename = "topCandidates",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  top_candidates: Vec<TopCandidate>,

  #[serde(
    rename = "chosenCandidates",
    skip_serializing_if = "Vec::is_empty",
    default
  )]
  chosen_candidates: Vec<Candidates>,

  #[serde(rename = "logProbabilitySum", skip_serializing_if = "Option::is_none")]
  log_probability_sum: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlContextMetadata {
  #[serde(rename = "urlMetadata", skip_serializing_if = "Vec::is_empty", default)]
  url_metadata: Vec<UrlMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlMetadata {
  #[serde(rename = "retrievedUrl", skip_serializing_if = "Option::is_none")]
  retrieved_url: Option<String>,

  #[serde(rename = "urlRetrievalStatus", skip_serializing_if = "Option::is_none")]
  url_retrieval_status: Option<UrlRetrievalStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopCandidate {
  #[serde(rename = "token", skip_serializing_if = "Vec::is_empty", default)]
  candidates: Vec<Candidates>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candidates {
  #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
  token: Option<String>,

  #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
  token_id: Option<i32>,

  #[serde(rename = "logProbability", skip_serializing_if = "Option::is_none")]
  log_probability: Option<f32>,
}
