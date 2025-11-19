use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub ai_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSourceConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub source: String, // "elasticsearch" or "openobserve"
    pub config: LogSourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub query: String,
    pub results: Vec<LogEntry>,
    pub total: usize,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    #[serde(default)]
    pub service: Option<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub logs: Vec<LogEntry>,
    pub question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub analysis: String,
    pub log_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugRequest {
    pub issue_description: String,
    #[serde(default)]
    pub context: DebugContext,
    pub config: DebugConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub source: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugContext {
    pub service: Option<String>,
    pub time_range: Option<String>,
    pub environment: Option<String>,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugResponse {
    pub issue: String,
    pub analysis: String,
    pub root_cause: Option<String>,
    pub recommendations: Vec<String>,
    pub relevant_logs: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugAnalysis {
    pub analysis: String,
    pub root_cause: Option<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub time_range: Option<String>,
    pub filters: HashMap<String, String>,
    pub size: usize,
}
