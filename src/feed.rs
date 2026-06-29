use serde::{ Serialize, Deserialize };
use url::Url;

#[deive(Serialize, Deserialize, Clone, Debug)]
pub enum Version {
    JSONFeed1_1
}

#[deive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {

}