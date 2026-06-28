use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Clone, Debug)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Url>
}