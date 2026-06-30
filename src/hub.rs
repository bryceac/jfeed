use serde::{ Deserialize, Serialize };
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hub {
    #[serde(rename = "type")]
    pub hub_type: String,
    pub url: Url
}

impl Hub {
    pub fn from(hub_type: &str, url: &str)
}