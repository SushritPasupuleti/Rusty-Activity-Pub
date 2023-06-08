use crate::AppState;
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
        pool_status: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "You've reached the backend API.".to_string(),
        pool_status: format!("Pool status: {:?}", state.db_pool.size()),
    };

    Json(api_data)
}
