use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod elasticsearch;
mod openobserve;
mod ai;
mod models;

use crate::{
    elasticsearch::ElasticsearchClient,
    openobserve::OpenObserveClient,
    ai::AIEngine,
    models::*,
};

#[derive(Clone)]
struct AppState {
    ai_engine: Arc<AIEngine>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,zeteo_backend=debug")
        .init();

    info!("🚀 Starting Logs Explorer Backend");

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize AI engine (must have OPENAI_API_KEY)
    let ai_engine = Arc::new(AIEngine::new_from_env()?);

    let state = AppState { ai_engine };

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health_check))
        .route("/api/query", post(query_logs))
        .route("/api/analyze", post(analyze_logs))
        .route("/api/debug", post(debug_issue))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = "0.0.0.0:3001";
    info!("✨ Server listening on http://{}", addr);
    info!("📝 API endpoints:");
    info!("   GET  / - Welcome message");
    info!("   GET  /api/health - Health check");
    info!("   POST /api/query - Query logs with AI");
    info!("   POST /api/analyze - Analyze logs");
    info!("   POST /api/debug - Debug issues");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "🔍 Logs Explorer API - AI-Powered Log Analysis\n\nEndpoints:\n  GET  /api/health\n  POST /api/query\n  POST /api/analyze\n  POST /api/debug"
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        ai_enabled: true,
    })
}

async fn query_logs(
    State(state): State<AppState>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, (StatusCode, String)> {
    info!("📥 Query request: {} (source: {})", request.query, request.source);

    // Create appropriate client based on source
    let logs = match request.source.as_str() {
        "elasticsearch" => {
            let client = ElasticsearchClient::new(
                &request.config.url,
                request.config.username.as_deref(),
                request.config.password.as_deref(),
            )
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let search_params = state.ai_engine
                .parse_query(&request.query)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            client.search(&search_params)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        }
        "openobserve" => {
            let client = OpenObserveClient::new(
                request.config.url.clone(),
                request.config.username.clone().unwrap_or_default(),
                request.config.password.clone().unwrap_or_default(),
                request.config.organization.clone().unwrap_or_else(|| "default".to_string()),
            )
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let search_params = state.ai_engine
                .parse_query(&request.query)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            client.search(&search_params)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unknown source: {}", request.source),
            ));
        }
    };

    info!("✅ Found {} log entries", logs.len());

    Ok(Json(QueryResponse {
        query: request.query,
        results: logs.clone(),
        total: logs.len(),
        summary: if !logs.is_empty() {
            Some(format!("Found {} logs", logs.len()))
        } else {
            Some("No logs found matching your query".to_string())
        },
    }))
}

async fn analyze_logs(
    State(state): State<AppState>,
    Json(request): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, String)> {
    info!("🔬 Analyzing {} logs", request.logs.len());

    let analysis = state.ai_engine
        .analyze_logs(&request.logs, &request.question)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AnalyzeResponse {
        analysis,
        log_count: request.logs.len(),
    }))
}

async fn debug_issue(
    State(state): State<AppState>,
    Json(request): Json<DebugRequest>,
) -> Result<Json<DebugResponse>, (StatusCode, String)> {
    info!("🐛 Debug request: {}", request.issue_description);

    // Create client
    let logs = match request.config.source.as_str() {
        "elasticsearch" => {
            let client = ElasticsearchClient::new(
                &request.config.url,
                request.config.username.as_deref(),
                request.config.password.as_deref(),
            )
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let search_params = state.ai_engine
                .create_debug_query(&request.issue_description, &request.context)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            client.search(&search_params).await
        }
        "openobserve" => {
            let client = OpenObserveClient::new(
                request.config.url.clone(),
                request.config.username.clone().unwrap_or_default(),
                request.config.password.clone().unwrap_or_default(),
                request.config.organization.clone().unwrap_or_else(|| "default".to_string()),
            )
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let search_params = state.ai_engine
                .create_debug_query(&request.issue_description, &request.context)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            client.search(&search_params).await
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unknown source: {}", request.config.source),
            ));
        }
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Analyze with AI
    let debug_analysis = state.ai_engine
        .debug_with_logs(&request.issue_description, &logs, &request.context)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    info!("✅ Debug analysis complete");

    Ok(Json(DebugResponse {
        issue: request.issue_description,
        analysis: debug_analysis.analysis,
        root_cause: debug_analysis.root_cause,
        recommendations: debug_analysis.recommendations,
        relevant_logs: logs,
    }))
}
