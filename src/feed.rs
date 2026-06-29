use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {
    pub version: Version
}