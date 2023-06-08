use axum::{response::IntoResponse, Json, extract::State};
use std::sync::Arc;
use crate::AppState;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn handler(
// State(state): State<Arc<AppState>>
    ) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "You've reached the backend API.".to_string(),
    };

    Json(
        api_data 
    )
}
