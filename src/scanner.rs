use crate::types::{TreeSitterQuery, MatchResult, CaptureResult, ApiResponse};
use crate::language_loader::get_language;
use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}, time::Duration, env};
use tree_sitter::{Parser, Query, QueryCursor, Tree};
use reqwest::{Client, header};
use serde_json::json;
use std::thread;

/// A scanner that caches parsed ASTs to avoid re-parsing files when new queries are received
pub struct Scanner {
    /// Cache of parsed ASTs by file path
    ast_cache: Arc<Mutex<HashMap<String, (Tree, String)>>>,
    /// HTTP client for API requests
    client: Client,
    /// API key for authentication
    api_key: String,
    /// Organization ID
    organization_id: String,
    /// Report ID for the current scan
    report_id: Option<String>,
    /// Code base version (commit hash)
    code_base_version: String,
    /// Base URL for API requests
    api_base_url: String,
}

impl Scanner {
    /// Create a new Scanner
    pub fn new(api_key: String, organization_id: String, code_base_version: String, report_id: Option<String>) -> Self {
        let client = Client::new();

        // Get the API base URL from environment variable or use default
        let api_base_url = env::var("API_BASE_URL").unwrap_or_else(|_| {
            "http://localhost:8080/api/v1".to_string()
        });

        Self {
            ast_cache: Arc::new(Mutex::new(HashMap::new())),
            client,
            api_key,
            organization_id,
            report_id,
            code_base_version,
            api_base_url,
        }
    }

    /// Initialize a code scan and get a report ID
    pub async fn initialize_code_scan(&mut self, file_types: Vec<String>, commit_hash: &str, branch_name: &str, repo_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // If we already have a report ID, return it
        if let Some(report_id) = &self.report_id {
            return Ok(report_id.clone());
        }

        // Create a CSV string of the file extensions
        let file_types_csv = file_types.join(",");
        println!("File extensions (CSV): {}", file_types_csv);

        let url = format!("{}/org/{}/rpc/initiate-code-scan-report/", self.api_base_url, self.organization_id);

        let request_body = json!({
            "file_types": file_types_csv,
            "commit_hash": commit_hash,
            "branch_name": branch_name,
            "repo_url": repo_url
        });

        println!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap());

        let response = self.client.post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("APIKey {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Error initializing code scan: {}", response.status()).into());
        }

        let data: ApiResponse = response.json().await?;
        println!("✅ Code scan initialized successfully. Report ID: {}", data.report_id);

        self.report_id = Some(data.report_id.clone());
        Ok(data.report_id)
    }

    /// Fetch available queries for the current report
    pub async fn fetch_available_queries(&self) -> Result<Vec<TreeSitterQuery>, Box<dyn std::error::Error>> {
        let report_id = self.report_id.as_ref().ok_or("No report ID available")?;

        println!("Fetching queries for report ID: {}", report_id);

        let url = format!(
            "{}/org/{}/rpc/get-preloaded-queries/{}",
            self.api_base_url,
            self.organization_id,
            report_id
        );

        let response = self.client.get(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("APIKey {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Error fetching queries: {}", response.status()).into());
        }

        let data: serde_json::Value = response.json().await?;
        let queries = data["TreeSitterQueries"].as_array()
            .ok_or("Invalid response format")?
            .iter()
            .map(|q| serde_json::from_value(q.clone()))
            .collect::<Result<Vec<TreeSitterQuery>, _>>()?;

        Ok(queries)
    }

    /// Post evidence to the API
    pub async fn post_evidence(&self, question_id: &str, evidence: Vec<CaptureResult>, query: &TreeSitterQuery) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/org/{}/evidence", self.api_base_url, self.organization_id);

        let request_body = json!({
            "question_id": question_id,
            "source_id": query.object_id,
            "source_type": "tree-sitter-query",
            "evidence": evidence,
            "evidence_context": query.reasoning
        });

        let response = self.client.post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("APIKey {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Error posting evidence: {}", response.status()).into());
        }

        println!("Evidence posted successfully");
        Ok(())
    }

    /// Parse a file and cache the AST
    pub fn parse_file(&self, file_path: &str, language_name: &str) -> Option<(Tree, String)> {
        let language = get_language(language_name)?;
        let path = PathBuf::from(file_path);
        let src = fs::read_to_string(&path).ok()?;

        let mut parser = Parser::new();
        parser.set_language(language).ok()?;
        let tree = parser.parse(&src, None)?;

        Some((tree, src))
    }

    /// Get the language for a file based on its extension
    pub fn get_language_for_file(&self, file_path: &str) -> Option<&'static str> {
        let extension = PathBuf::from(file_path)
            .extension()?
            .to_str()?
            .to_lowercase();

        match extension.as_str() {
            "rs" => Some("rust"),
            "js" => Some("javascript"),
            "py" => Some("python"),
            "go" => Some("go"),
            "ts" => Some("typescript"),
            "java" => Some("java"),
            "cpp" | "h" | "hpp" | "cc" => Some("cpp"),
            "rb" => Some("ruby"),
            "php" => Some("php"),
            _ => None,
        }
    }

    /// Run a query on a tree and return the matches
    pub fn run_query_on_tree(&self, tree: &Tree, source: &str, query_text: &str, language_name: &str) -> Vec<CaptureResult> {
        let language = match get_language(language_name) {
            Some(lang) => lang,
            None => return vec![],
        };

        let query = match Query::new(language, query_text) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to compile query: {}", e);
                return vec![];
            }
        };

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

        let mut results = Vec::new();
        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let start = node.start_position();
                let text = &source[node.start_byte()..node.end_byte()];

                let capture_name = match query.capture_names().get(capture.index as usize) {
                    Some(name) => name.clone(),
                    None => format!("capture_{}", capture.index),
                };

                results.push(CaptureResult {
                    name: capture_name,
                    value: text.to_string(),
                    position: (start.row + 1, start.column + 1),
                    node_type: node.kind().to_string(),
                });
            }
        }

        results
    }

    /// Scan files with the given queries
    pub async fn scan_files(&self, files: Vec<String>, queries: Vec<TreeSitterQuery>) -> Vec<MatchResult> {
        let mut results = Vec::new();
        let mut cache = self.ast_cache.lock().unwrap();

        // Group queries by file type
        let queries_by_type: HashMap<String, Vec<&TreeSitterQuery>> = queries.iter()
            .fold(HashMap::new(), |mut acc, q| {
                acc.entry(q.file_type.clone()).or_insert_with(Vec::new).push(q);
                acc
            });

        // Process each file
        for file_path in &files {
            let lang_name = match self.get_language_for_file(file_path) {
                Some(lang) => lang,
                None => continue,
            };

            println!("📄 Scanning: {}", file_path);

            // Check if the file is already in the cache
            let (tree, source) = if let Some(cached) = cache.get(file_path) {
                println!("Using cached AST for {}", file_path);
                cached.clone()
            } else {
                // Parse the file and add it to the cache
                match self.parse_file(file_path, lang_name) {
                    Some((tree, src)) => {
                        println!("Parsed and cached AST for {}", file_path);
                        let result = (tree, src);
                        cache.insert(file_path.clone(), result.clone());
                        result
                    },
                    None => {
                        eprintln!("Failed to parse {}", file_path);
                        continue;
                    }
                }
            };

            // Get relevant queries for this file type
            let file_ext = PathBuf::from(file_path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| format!(".{}", ext.to_lowercase()))
                .unwrap_or_default();

            let relevant_queries = queries_by_type.get(&file_ext).cloned().unwrap_or_default();
            println!("🔍 Found {} relevant queries for {}", relevant_queries.len(), file_path);

            // Process each query
            for query in relevant_queries {
                let captures = self.run_query_on_tree(&tree, &source, &query.query, lang_name);

                if !captures.is_empty() {
                    // println!("⚠️ Matched Rule: {} — {}", query.object_id, query.prompt);

                    for capture in &captures {
                        // println!("  📌 {}: \"{}\" @ line {}", capture.name, capture.value, capture.position.0);

                        results.push(MatchResult {
                            file: file_path.clone(),
                            line: capture.position.0,
                            column: capture.position.1,
                            text: capture.value.clone(),
                            question_id: query.question_id.clone(),
                            organization_id: self.organization_id.clone(),
                            code_base_version: self.code_base_version.clone(),
                        });
                    }
                } else {
                    // println!("✅ No matches for rule {}", query.prompt);
                }
            }
        }

        results
    }

    /// Start a continuous scan that polls for new queries
    pub async fn start_continuous_scan(&self, files: Vec<String>, poll_interval_secs: u64, max_polls: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut poll_count = 0;

        while poll_count < max_polls {
            println!("Polling for new queries...");

            // Fetch available queries
            let queries = self.fetch_available_queries().await?;
            println!("Available Queries: {:?}", queries);

            // Scan files with the fetched queries
            let results = self.scan_files(files.clone(), queries.clone()).await;

            // Post evidence for each query
            for query in &queries {
                let evidence: Vec<CaptureResult> = results.iter()
                    .filter(|r| r.question_id == query.question_id)
                    .map(|r| CaptureResult {
                        name: "match".to_string(),
                        value: r.text.clone(),
                        position: (r.line, r.column),
                        node_type: "unknown".to_string(),
                    })
                    .collect();

                if evidence.is_empty() {
                    // If no matches, still post a "no matches" evidence
                    let no_match = CaptureResult {
                        name: "no_match".to_string(),
                        value: "No matches found".to_string(),
                        position: (0, 0),
                        node_type: "none".to_string(),
                    };
                    self.post_evidence(&query.question_id, vec![no_match], query).await?;
                } else {
                    self.post_evidence(&query.question_id, evidence, query).await?;
                }
            }

            poll_count += 1;

            // Sleep before the next poll
            if poll_count < max_polls {
                thread::sleep(Duration::from_secs(poll_interval_secs));
            }
        }

        Ok(())
    }
}
