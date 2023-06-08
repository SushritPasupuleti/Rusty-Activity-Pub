use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

use crate::{handlers, AppState};

pub fn api_routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new().route("/test-api", get(handlers::api::handler))
        .with_state(state)
}
