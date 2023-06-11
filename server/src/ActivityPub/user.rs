use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct User {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub name: String,
    pub preferredUsername: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub discoverable: Option<bool>,
}
