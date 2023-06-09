use crate::{middleware::auth, handlers::{users::get_user_by_id}};
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

use crate::{handlers, AppState};

pub fn api_routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/:id", get(get_user_by_id))
        .with_state(state)
}
