use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use tracing::{info, debug};

use crate::models::{LogEntry, SearchParams};

pub struct OpenObserveClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
    organization: String,
}

impl OpenObserveClient {
    pub fn new(
        base_url: String,
        username: String,
        password: String,
        organization: String,
    ) -> Result<Self> {
        let client = Client::new();
        
        info!("OpenObserve client initialized for {}", base_url);
        Ok(Self {
            client,
            base_url,
            username,
            password,
            organization,
        })
    }

    pub async fn search(&self, params: &SearchParams) -> Result<Vec<LogEntry>> {
        debug!("Searching OpenObserve with params: {:?}", params);

        let sql_query = self.build_sql_query(params);

        let url = format!(
            "{}/api/{}/default/_search",
            self.base_url.trim_end_matches('/'), 
            self.organization
        );

        let body = json!({
            "query": {
                "sql": sql_query,
                "size": params.size
            }
        });

        debug!("OpenObserve request URL: {}", url);
        debug!("OpenObserve SQL query: {}", sql_query);

        let response = self.client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow::anyhow!(
                "OpenObserve API error {}: {}",
                status,
                body
            ));
        }

        let response_body: serde_json::Value = response.json().await?;
        
        let hits = response_body["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        let logs: Vec<LogEntry> = hits
            .iter()
            .filter_map(|hit| {
                // Normalize OpenObserve response to LogEntry
                let mut entry = hit.clone();
                if let Some(obj) = entry.as_object_mut() {
                    if !obj.contains_key("level") {
                        obj.insert("level".to_string(), json!("INFO"));
                    }
                    if !obj.contains_key("message") && obj.contains_key("log") {
                        if let Some(log) = obj.get("log").cloned() {
                            obj.insert("message".to_string(), log);
                        }
                    }
                }
                serde_json::from_value(entry).ok()
            })
            .collect();

        info!("Found {} log entries from OpenObserve", logs.len());
        Ok(logs)
    }

    fn build_sql_query(&self, params: &SearchParams) -> String {
        let mut conditions = vec![];

        // Add text search
        if !params.query.is_empty() {
            conditions.push(format!("(message LIKE '%{}%' OR log LIKE '%{}%')", 
                params.query, params.query));
        }

        // Add time range
        if let Some(time_range) = &params.time_range {
            let time_value = self.parse_time_range(time_range);
            conditions.push(format!("_timestamp > {}", time_value));
        }

        // Add filters
        for (field, value) in &params.filters {
            conditions.push(format!("{} = '{}'", field, value));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        format!(
            "SELECT * FROM default {} ORDER BY _timestamp DESC LIMIT {}",
            where_clause, params.size
        )
    }

    fn parse_time_range(&self, range: &str) -> i64 {
        // Parse ranges like "1h", "24h", "7d" to milliseconds ago
        let now = chrono::Utc::now().timestamp_millis();
        let duration = match range.chars().last() {
            Some('h') => {
                let hours: i64 = range[..range.len() - 1].parse().unwrap_or(1);
                hours * 3600 * 1000
            }
            Some('d') => {
                let days: i64 = range[..range.len() - 1].parse().unwrap_or(1);
                days * 86400 * 1000
            }
            Some('m') => {
                let minutes: i64 = range[..range.len() - 1].parse().unwrap_or(5);
                minutes * 60 * 1000
            }
            _ => 3600 * 1000, // Default: 1 hour
        };
        now - duration
    }
}
