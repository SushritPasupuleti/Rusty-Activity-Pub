use crate::{middleware::auth, handlers::{webfinger::get_web_finger}};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

use crate::{handlers, AppState};

pub fn api_routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/", get(get_web_finger))
        .with_state(state)
}
