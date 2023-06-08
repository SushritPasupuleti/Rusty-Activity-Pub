use axum::{Router, Json, response::IntoResponse, routing::get};

use crate::{
    handlers,
};

pub fn api_routes() -> Router {
    Router::new()
        .route("/test-api", get(handlers::api::handler))
}
