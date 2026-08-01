//! REST API routes using Axum.

use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub version: String,
}

/// Create the Axum router with all API routes.
pub fn create_router() -> Router {
    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
    });

    Router::new()
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/info", get(system_info))
        .route("/api/v1/nodes", get(list_nodes))
        .route("/api/v1/network/stats", get(network_stats))
        .route("/api/v1/storage/stats", get(storage_stats))
        .route("/api/v1/ai/models", get(list_models))
        .route("/api/v1/compute/tasks", get(list_tasks))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn system_info(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "name": "WIOS",
        "version": state.version,
        "platform": wios_core::types::Platform::current().to_string(),
        "uptime_secs": 0,
    }))
}

async fn list_nodes() -> Json<Value> {
    Json(json!({
        "nodes": [],
        "total": 0,
    }))
}

async fn network_stats() -> Json<Value> {
    Json(json!({
        "connected_peers": 0,
        "bytes_sent": 0,
        "bytes_received": 0,
        "messages_sent": 0,
        "messages_received": 0,
    }))
}

async fn storage_stats() -> Json<Value> {
    Json(json!({
        "total_keys": 0,
        "total_bytes": 0,
        "namespaces": [],
    }))
}

async fn list_models() -> Json<Value> {
    Json(json!({
        "models": [],
        "total": 0,
    }))
}

async fn list_tasks() -> Json<Value> {
    Json(json!({
        "tasks": [],
        "total": 0,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_router();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_system_info() {
        let app = create_router();
        let response = app
            .oneshot(Request::builder().uri("/api/v1/info").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
