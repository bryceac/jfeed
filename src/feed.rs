use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Version {
    JSONFeed1_1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {

}