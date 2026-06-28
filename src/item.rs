use chrono::prelude::*;
use serde::{ Serialize, Deserialize };
use url::Url;
use crate::Author;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    id: String,
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(flatten)]
    content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(rename = "content_html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "content_text", skip_serializing_if ="Option::is_none")]
    pub text: Option<String>
}