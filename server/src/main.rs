use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use base64;
use hex;
use ring::rand::SecureRandom;
use ring::{digest, hmac, rand};
use sha2::{Digest, Sha256, Sha512};
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
        .route("/sha256", get(getSHA256))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is Running and Listening on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Sha256Result {
    type_: String,
    message: String,
    sha256: String,
    hex: String,
    base64: String,
    key: Option<String>,
}

async fn getSHA256() -> impl IntoResponse {
    let message = b"hello world";

    let mut hasher = Sha256::new();

    hasher.update(message);

    let result = hasher.finalize();

    //conversions
    let hex_res = hex::encode(result);
    let base64_res = base64::encode(result);

    let res = format!(
        "message: {:?}\n sha256: {:?}\n hex: {}\n base64: {}",
        message, result, hex_res, base64_res
    );

    let sha_res = Sha256Result {
        type_: "SHA256".to_string(),
        message: String::from_utf8(message.to_vec()).unwrap(),
        sha256: format!("{:?}", result),
        hex: hex_res,
        base64: base64_res,
        key: None,
    };

    println!("Res: {}: ", res);

    // let mut key_value = [0u8; 48];
    let mut key_value = "secret".as_bytes().to_vec();

    println!("key_value: {:?}", key_value);

    let rng = rand::SystemRandom::new();
    //don't seed random here as we will use PrivateKey to seed it
    // rng.fill(&mut key_value).unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let signature = hmac::sign(&key, message);

    let res2 = format!(
        "message: {:?}\n signature: {:?}\n hex: {}\n base64: {}",
        message,
        signature,
        hex::encode(signature),
        base64::encode(signature)
    );

    let signed_res = Sha256Result {
        type_: "Signed".to_string(),
        message: String::from_utf8(message.to_vec()).unwrap(),
        sha256: format!("{:?}", signature),
        hex: hex::encode(signature),
        base64: base64::encode(signature),
        key: Some(String::from_utf8(key_value).unwrap()),
    };

    let res_array = vec![sha_res, signed_res];

    println!("Res2: {}: ", res2);

    (StatusCode::OK, Json(res_array))
}
