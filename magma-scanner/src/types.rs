use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct TreeSitterQuery {
    pub question_id: String,
    pub file_type: String,
    pub query: String,
    #[serde(default)]
    pub object_id: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub reasoning: String,
}

#[derive(Debug, Deserialize)]
pub struct InputData {
    #[serde(rename = "filesByType")]
    pub files_by_type: HashMap<String, Vec<String>>,
    pub queries: Vec<TreeSitterQuery>,
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    #[serde(rename = "codeBaseVersion")]
    pub code_base_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResult {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
    pub question_id: String,
    pub organization_id: String,
    pub code_base_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptureResult {
    pub name: String,
    pub value: String,
    pub position: (usize, usize), // (line, column)
    pub node_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub question_id: String,
    pub source_id: String,
    pub source_type: String,
    pub evidence: Vec<CaptureResult>,
    pub evidence_context: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub report_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PosInputData {
    pub api_key: String,
    pub organization_id: String,
    pub code_base_version: String,
    pub report_id: Option<String>,
    pub target_dir: String,
    pub poll_interval_secs: Option<u64>,
    pub max_polls: Option<usize>,
}
