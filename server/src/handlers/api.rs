use crate::AppState;
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn test_api(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
        pool_status: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "API Version 0.0.1".to_string(),
        pool_status: format!("Pool status: {:?}", state.db_pool.size()),
    };

    Json(api_data)
}
pub async fn test_auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
        pool_status: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "Checked Auth - API Version 0.0.1".to_string(),
        pool_status: format!("Pool status: {:?}", state.db_pool.size()),
    };

    Json(api_data)
}

