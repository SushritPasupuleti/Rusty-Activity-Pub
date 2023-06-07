use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        // .allow_credentials()
        ;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is Running and Listening on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
