use serde::{ Serialize, Deserialize };
use url::Url;
use crate::{FeedVersion, Author, Item};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {
    pub version: Url,
    pub title: String,
    pub homepage: Url,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<Url>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "expired_is_default")]
    pub expired: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hubs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Item>
}

fn expired_is_default(expired: &bool) -> bool {
    *expired == bool::default()
}