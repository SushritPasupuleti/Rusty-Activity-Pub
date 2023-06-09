use crate::{
    models::accounts::{Account, AccountWebFinger, AccountActor},
    ActivityPub::{webfinger::{Links, Webfinger}, user::User},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct GetUserQuery {
    username: String,
}

pub async fn get_user_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("id: {:?}", id);

    if id.is_empty() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }

    let host_url = &state.host_url;

    let users = sqlx::query_as::<_, AccountActor>("SELECT name from accounts where name = $1")
        .bind(id)
        .fetch_all(&state.db_pool)
        .await
        .expect("Not found");

    println!("users: {:?}", users);

    if users.len() == 0 {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    }

    let user = &users[0];

    let user_result = User {
        context: vec![
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string(),
        ],
        id: format!("{}/users/{}", host_url, user.name),
        name: user.name.clone(),
        r#type: "Person".to_string(),
        inbox: format!("{}/users/{}/inbox", host_url, user.name),
        followers: format!("{}/users/{}/followers", host_url, user.name),
        following: format!("{}/users/{}/following", host_url, user.name),
        discoverable: None,
        preferredUsername: None,
        summary: None,
        url: None,
    };

    (StatusCode::OK, Json(json!(user_result)))
}
