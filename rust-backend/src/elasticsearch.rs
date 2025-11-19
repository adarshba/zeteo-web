use anyhow::Result;
use elasticsearch::{Elasticsearch, SearchParts, http::transport::Transport};
use serde_json::json;
use tracing::{info, debug};

use crate::models::{LogEntry, SearchParams};

pub struct ElasticsearchClient {
    client: Elasticsearch,
}

impl ElasticsearchClient {
    pub fn new(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Self> {
        let transport = if let (Some(user), Some(pass)) = (username, password) {
            let auth_url = if url.contains("@") {
                url.to_string()
            } else {
                let base_url = url.replace("http://", "").replace("https://", "");
                let protocol = if url.starts_with("https") { "https" } else { "http" };
                format!("{}://{}:{}@{}", protocol, user, pass, base_url)
            };
            Transport::single_node(&auth_url)?
        } else {
            Transport::single_node(url)?
        };

        let client = Elasticsearch::new(transport);
        
        info!("Elasticsearch client initialized for {}", url);
        Ok(Self { client })
    }

    pub async fn search(&self, params: &SearchParams) -> Result<Vec<LogEntry>> {
        debug!("Searching Elasticsearch with params: {:?}", params);

        let mut must_clauses = vec![];

        // Add query string
        if !params.query.is_empty() {
            must_clauses.push(json!({
                "query_string": {
                    "query": params.query,
                    "fields": ["message", "log", "msg", "*"]
                }
            }));
        }

        // Add time range
        if let Some(time_range) = &params.time_range {
            must_clauses.push(json!({
                "range": {
                    "@timestamp": {
                        "gte": format!("now-{}", time_range)
                    }
                }
            }));
        }

        // Add filters
        for (field, value) in &params.filters {
            must_clauses.push(json!({
                "term": {
                    field: value
                }
            }));
        }

        let search_body = json!({
            "query": {
                "bool": {
                    "must": if must_clauses.is_empty() {
                        vec![json!({"match_all": {}})]
                    } else {
                        must_clauses
                    }
                }
            },
            "size": params.size,
            "sort": [
                { "@timestamp": { "order": "desc" } }
            ]
        });

        let response = self.client
            .search(SearchParts::Index(&["logs-*", "filebeat-*", "logstash-*"]))
            .body(search_body)
            .send()
            .await?;

        let response_body = response.json::<serde_json::Value>().await?;
        
        let hits = response_body["hits"]["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        let logs: Vec<LogEntry> = hits
            .iter()
            .filter_map(|hit| {
                let mut source = hit["_source"].clone();
                
                // Normalize field names
                if let Some(obj) = source.as_object_mut() {
                    // Ensure required fields exist
                    if !obj.contains_key("timestamp") && obj.contains_key("@timestamp") {
                        if let Some(ts) = obj.get("@timestamp").cloned() {
                            obj.insert("timestamp".to_string(), ts);
                        }
                    }
                    if !obj.contains_key("level") {
                        obj.insert("level".to_string(), json!("INFO"));
                    }
                    if !obj.contains_key("message") {
                        if let Some(msg) = obj.get("log").or_else(|| obj.get("msg")) {
                            obj.insert("message".to_string(), msg.clone());
                        } else {
                            obj.insert("message".to_string(), json!(""));
                        }
                    }
                }
                
                serde_json::from_value(source).ok()
            })
            .collect();

        info!("Found {} log entries from Elasticsearch", logs.len());
        Ok(logs)
    }
}
