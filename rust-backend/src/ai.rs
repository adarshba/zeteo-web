use anyhow::Result;
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::models::{DebugAnalysis, DebugContext, LogEntry, SearchParams};

pub struct AIEngine {
    client: Client<async_openai::config::OpenAIConfig>,
    model: String,
}

impl AIEngine {
    pub fn new(api_key: String, model: Option<String>) -> Result<Self> {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        let model = model.unwrap_or_else(|| "gpt-4o-mini".to_string());

        info!("AI Engine initialized with model: {}", model);
        Ok(Self { client, model })
    }

    pub fn new_from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable not set"))?;
        let model = std::env::var("OPENAI_MODEL").ok();
        Self::new(api_key, model)
    }

    pub async fn parse_query(&self, query: &str) -> Result<SearchParams> {
        debug!("Parsing query with AI: {}", query);

        let system_prompt = r#"You are a log query parser. Convert natural language queries into structured search parameters.
Extract:
1. The main search query (keywords to search in logs)
2. Time range if mentioned (format: 1h, 24h, 7d, etc.)
3. Filters like service name, log level, etc.

Respond in JSON format:
{
  "query": "extracted keywords",
  "time_range": "1h" or null,
  "filters": {"field": "value"}
}

Examples:
- "Show errors in payment service last hour" -> {"query": "error", "time_range": "1h", "filters": {"service": "payment", "level": "ERROR"}}
- "Database timeouts" -> {"query": "database timeout", "time_range": null, "filters": {}}
- "Recent logs" -> {"query": "", "time_range": "1h", "filters": {}}
"#;

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(query)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.3)
            .build()?;

        let response = self.client.chat().create(request).await?;

        let content = response.choices[0]
            .message
            .content
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No response from AI"))?;

        // Parse JSON response
        let parsed: serde_json::Value = serde_json::from_str(content)
            .unwrap_or_else(|_| serde_json::json!({
                "query": query,
                "time_range": "1h",
                "filters": {}
            }));

        Ok(SearchParams {
            query: parsed["query"].as_str().unwrap_or(query).to_string(),
            time_range: parsed["time_range"].as_str().map(|s| s.to_string()),
            filters: parsed["filters"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            size: 100,
        })
    }

    pub async fn analyze_logs(&self, logs: &[LogEntry], question: &str) -> Result<String> {
        info!("Analyzing {} logs", logs.len());

        let logs_json = serde_json::to_string_pretty(logs)?;

        let system_prompt = "You are a log analysis expert. Analyze the provided logs and answer the user's question with detailed insights, patterns, and actionable recommendations.";

        let user_prompt = format!(
            "Question: {}\n\nLogs:\n{}\n\nProvide a detailed analysis.",
            question, logs_json
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.5)
            .max_tokens(1500u16)
            .build()?;

        let response = self.client.chat().create(request).await?;

        Ok(response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_else(|| "Unable to analyze logs".to_string()))
    }

    pub async fn create_debug_query(
        &self,
        issue: &str,
        context: &DebugContext,
    ) -> Result<SearchParams> {
        debug!("Creating debug query for issue: {}", issue);

        let context_str = format!(
            "Service: {:?}, Time: {:?}, Env: {:?}, User: {:?}, Request: {:?}",
            context.service,
            context.time_range,
            context.environment,
            context.user_id,
            context.request_id
        );

        let system_prompt = r#"You are a debugging expert. Based on the issue description and context, create an effective search query to find relevant logs.

Respond in JSON format:
{
  "query": "search keywords",
  "time_range": "suggested time range",
  "filters": {"field": "value"}
}
"#;

        let user_prompt = format!(
            "Issue: {}\nContext: {}\n\nCreate an optimal search query.",
            issue, context_str
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.3)
            .build()?;

        let response = self.client.chat().create(request).await?;

        let content = response.choices[0]
            .message
            .content
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No response from AI"))?;

        let parsed: serde_json::Value = serde_json::from_str(content)
            .unwrap_or_else(|_| serde_json::json!({
                "query": issue,
                "time_range": context.time_range.clone().unwrap_or_else(|| "1h".to_string()),
                "filters": {}
            }));

        let mut filters: HashMap<String, String> = parsed["filters"]
            .as_object()
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        // Add context filters
        if let Some(service) = &context.service {
            filters.insert("service".to_string(), service.clone());
        }

        Ok(SearchParams {
            query: parsed["query"].as_str().unwrap_or(issue).to_string(),
            time_range: Some(
                parsed["time_range"]
                    .as_str()
                    .unwrap_or("1h")
                    .to_string(),
            ),
            filters,
            size: 200,
        })
    }

    pub async fn debug_with_logs(
        &self,
        issue: &str,
        logs: &[LogEntry],
        context: &DebugContext,
    ) -> Result<DebugAnalysis> {
        info!("Performing AI-powered debug analysis");

        let logs_summary = if logs.len() > 50 {
            // Summarize if too many logs
            format!("Found {} logs. Here are the most recent:\n{}", 
                logs.len(),
                serde_json::to_string_pretty(&logs[..50])?
            )
        } else {
            serde_json::to_string_pretty(logs)?
        };

        let context_str = serde_json::to_string_pretty(context)?;

        let system_prompt = r#"You are an expert SRE and debugging assistant. Analyze the issue, logs, and context to provide:
1. A detailed analysis of what's happening
2. The root cause (if identifiable)
3. Step-by-step recommendations to fix the issue

Respond in JSON format:
{
  "analysis": "detailed analysis",
  "root_cause": "identified root cause or null",
  "recommendations": ["step 1", "step 2", ...]
}
"#;

        let user_prompt = format!(
            "Issue: {}\n\nContext:\n{}\n\nLogs:\n{}\n\nProvide debugging analysis.",
            issue, context_str, logs_summary
        );

        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?,
            ),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.4)
            .max_tokens(2000u16)
            .build()?;

        let response = self.client.chat().create(request).await?;

        let content = response.choices[0]
            .message
            .content
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No response from AI"))?;

        let parsed: serde_json::Value = serde_json::from_str(content)
            .unwrap_or_else(|_| serde_json::json!({
                "analysis": content,
                "root_cause": null,
                "recommendations": []
            }));

        Ok(DebugAnalysis {
            analysis: parsed["analysis"]
                .as_str()
                .unwrap_or(content)
                .to_string(),
            root_cause: parsed["root_cause"]
                .as_str()
                .map(|s| s.to_string()),
            recommendations: parsed["recommendations"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}
