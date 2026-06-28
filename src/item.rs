use chrono::prelude::*;
use serde::{ Serialize, Deserialize };
use url::Url;
use crate::Author;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    id: String,
    url: Url,
    external_url: Option<Url>,
    title: Option<String>,
    #[serde(flatten)]
    content: Option<Content>,
    summary: Option<String>,
    image: Option<Url>,
    banner: Option<Url>,
    published: Option<DateTime<Utc>>,
    modified: Option<DateTime<Utc>>,
    authors: Vec<Author>,
    tags: Vec<String>,
    language: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(rename = "content_html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "content_text", skip_serializing_if ="Option::is_none")]
    pub text: Option<String>
}