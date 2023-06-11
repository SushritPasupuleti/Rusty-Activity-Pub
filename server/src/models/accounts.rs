use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Account {
    name: String,
    privkey: String,
    pubkey: String,
    webfinger: String,
    actor: String,
    apikey: String,
    followers: String,
    messages: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct AccountWebFinger {
    pub webfinger: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct AccountActor {
    pub name: String,
}
