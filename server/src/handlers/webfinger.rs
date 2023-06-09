use crate::{
    models::accounts::{Account, AccountWebFinger},
    ActivityPub::webfinger::{Links, Webfinger},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct GetUserQuery {
    resource: String,
}

pub async fn get_web_finger(
    query: axum::extract::Query<GetUserQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let resource: &str = query.resource.as_str();

    let host_url = &state.host_url;

    println!("resource: {:?}", resource);

    let _webfinger_result = if resource.contains("acct:") {
        let resource = &resource.replace("acct:", "");

        println!("resource: {:?}", resource);

        let accounts =
            sqlx::query_as::<_, AccountWebFinger>("SELECT webfinger from accounts where name = $1")
                .bind(resource)
                .fetch_all(&state.db_pool)
                .await
                .expect("Not found");

        println!("accounts: {:?}", accounts);

        if accounts.len() == 0 {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
        }

        let webfinger = Webfinger {
            subject: format!("acct:{}", resource),
            links: vec![Links {
                rel: "self".to_string(),
                r#type: "application/activity+json".to_string(),
                href: format!("{}/users/{}", host_url, accounts[0].webfinger),
            }],
        };

        println!("webfinger: {:?}", webfinger);

        return (StatusCode::OK, Json(json!(webfinger)));
    } else {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    };
}
