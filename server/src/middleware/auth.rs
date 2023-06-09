use axum::handler::Handler;
use axum::http;
use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde_json::json;

use crate::handlers::api::Claims;

#[derive(Clone)]
struct CurrentUser {
    name: String,
}

pub async fn authorization_check<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    println!("auth_header: {:?}", auth_header);

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    let auth_token = auth_token.trim_start_matches("Bearer ");

    println!("auth_token parsed: {}", auth_token);

    let secret = "secret for JWT";

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&[""]);

    println!("validation: {:?}", validation);

    let decoded = decode::<Claims>(
        auth_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|err| {
        println!("err: {:?}", err);
    })
    .ok();

    if decoded.is_none() {
        return None;
    }

    // pretend inserted
    println!("auth_token: {}, decoded: {:?}", auth_token, decoded);
    Some(CurrentUser {
        name: "foo".to_string(),
    })
}
