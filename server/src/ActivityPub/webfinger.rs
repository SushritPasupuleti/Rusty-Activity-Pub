use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Webfinger {
    pub subject: String,
    pub links: Vec<Links>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Links {
    pub rel: String,
    pub r#type: String,
    pub href: String,
}
