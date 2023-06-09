#[allow(non_snake_case)]
mod ActivityPub;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use crate::models::accounts::Account;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use base64;
use hex;

use ::rand::Rng;
use argon2::Config;
use rcgen::{date_time_ymd, Certificate, CertificateParams, DistinguishedName};
use ring::rand::SecureRandom;
use ring::{digest, hmac, rand};
use sha2::{Digest, Sha256, Sha512};
use sqlx::postgres::PgPoolOptions;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    db_pool: sqlx::PgPool,
    host_name: String,
    host_url: String,
}

impl AppState {
    pub fn new(db_pool: sqlx::PgPool, host_name: String, host_url: String) -> Self {
        Self {
            db_pool,
            host_name,
            host_url,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    //db setup
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let host_name = std::env::var("HOST_NAME").expect("Unable to read HOST_NAME env var");
    let host_url = std::env::var("HOST_URL").expect("Unable to read HOST_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    //run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations");

    let state = Arc::new(AppState {
        db_pool: pool,
        host_name,
        host_url,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        // .allow_credentials()
        ;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/sha256", get(get_sha256))
        .route("/accounts", get(get_accounts))
        // .merge(routes::api::api_routes().with_state(state))
        .with_state(state.clone())
        .nest("/api", routes::api::api_routes(state.clone()))
        .nest(
            "/.well-known/webfinger",
            routes::webfinger::api_routes(state.clone()),
        )
        .nest("/users", routes::users::api_routes(state.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is Running and Listening on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(utils::server::shutdown_signal())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashed_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashed_private_key: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ShaRequestQuery {
    message: String,
    key: String,
}

//just for testing Rust's crypto libraries
async fn get_sha256(
    query: axum::extract::Query<ShaRequestQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let message = query.message.as_bytes();

    println!("message: {:?}", query.message);

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
        public_key: None,
        private_key: None,
        hashed_public_key: None,
        hashed_private_key: None,
    };

    println!("Res: {}: ", res);

    // let mut key_value = b"".as_bytes().to_vec();
    let key_value: Vec<u8> = query.key.as_bytes().to_vec();

    println!("secret: {:?}, key_value: {:?}", query.key, key_value);

    // let rng = rand::SystemRandom::new();
    //don't seed random here as we will use PrivateKey to seed it
    // rng.fill(&mut key_value).unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let signature = hmac::sign(&key, message);

    let modulus_len = 4096;

    let pkey: openssl::pkey::PKey<_> = openssl::rsa::Rsa::generate(modulus_len)
        .unwrap()
        .try_into()
        .unwrap();
    let key_pair_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();
    let key_pair = rcgen::KeyPair::from_pem(&key_pair_pem).unwrap();

    println!("(1)key_pair: {:?}", key_pair);

    let salt = ::rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    let hashed_pub_key =
        argon2::hash_encoded(key_pair.public_key_pem().as_bytes(), &salt, &config).unwrap();

    let hashed_priv_key =
        argon2::hash_encoded(key_pair.serialize_pem().as_bytes(), &salt, &config).unwrap();

    let verify_hashed_pub_key = argon2::verify_encoded(&hashed_pub_key, key_pair.public_key_pem().as_bytes()).unwrap();
    let verify_hashed_priv_key = argon2::verify_encoded(&hashed_priv_key, key_pair.serialize_pem().as_bytes()).unwrap();

    println!("hashed_pub_key: {:?}", hashed_pub_key);
    println!("hashed_priv_key: {:?}", hashed_priv_key);
    println!("verify_hashed_pub_key: {:?}", verify_hashed_pub_key);
    println!("verify_hashed_priv_key: {:?}", verify_hashed_priv_key);
    println!("salt: {:?}", salt);

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
        public_key: Some(key_pair.public_key_pem()),
        private_key: Some(key_pair.serialize_pem()),
        hashed_public_key: Some(hashed_pub_key),
        hashed_private_key: Some(hashed_priv_key),
        // public_key: None,
        // private_key: None,
    };

    let res_array = vec![sha_res, signed_res];

    println!("Res2: {}: ", res2);

    (StatusCode::OK, Json(res_array))
}

async fn get_accounts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.db_pool;

    let accounts_struct = sqlx::query_as::<_, Account>(r"SELECT * FROM accounts")
        .fetch_all(pool)
        .await
        .expect("Unable to fetch accounts");

    (StatusCode::OK, Json(accounts_struct))
}
