use crate::middleware::auth;
use axum::{extract::State, middleware, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

use crate::{handlers, AppState};

pub fn api_routes<S>(state: Arc<AppState>) -> Router<S> {
    Router::new()
        .route("/test-auth", get(handlers::api::test_auth))
        //auth middleware is applied to all above next line.
        .route_layer(middleware::from_fn(auth::authorization_check))
        .route("/test-api", get(handlers::api::test_api))
        .route("/gen-jwt", get(handlers::api::gen_jwt))
        .with_state(state)
}
