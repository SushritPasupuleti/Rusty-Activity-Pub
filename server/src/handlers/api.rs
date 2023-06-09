use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn test_api(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
        pool_status: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "API Version 0.0.1".to_string(),
        pool_status: format!("Pool status: {:?}", state.db_pool.size()),
    };

    Json(api_data)
}
pub async fn test_auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Seeking api data");

    #[derive(serde::Serialize)]
    struct _ApiData {
        result: String,
        message: String,
        pool_status: String,
    }

    let api_data = _ApiData {
        result: "ok".to_string(),
        message: "Checked Auth - API Version 0.0.1".to_string(),
        pool_status: format!("Pool status: {:?}", state.db_pool.size()),
    };

    Json(api_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// local_user_id, standard claim by RFC 7519.
    pub sub: i32,
    pub iss: String,
    /// Time when this token was issued as UNIX-timestamp in seconds
    pub iat: i64,
    // pub exp: Option<i64>,
}

pub async fn gen_jwt(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Generating JWT");
    println!("Generating JWT");

    let claims = Claims {
        sub: 1,
        iss: "https://example.com".to_string(),
        iat: 1234567890,
        // exp: None,
    };

    let key: EncodingKey = EncodingKey::from_secret("secret for JWT".as_ref());

    let token = encode(&Header::default(), &claims, &key).unwrap();

    println!("Token: {}", token);

    #[derive(serde::Serialize)]
    struct Result {
        token: String,
    }

    let result = Result {
        token: token.to_string(),
    };

    (StatusCode::OK, Json(result))
}

