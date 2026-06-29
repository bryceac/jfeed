use serde::{ Serialize, Deserialize };
use url::Url;
use crate::{ Author, Content };

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>
}