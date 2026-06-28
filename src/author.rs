use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Clone, Debug)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<Url>,
    pub avatar: Option<Url>
}