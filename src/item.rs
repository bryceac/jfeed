use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    id: String,
    url: Url,
    external_url: Option<Url>,
    title: Option<String>,
    content_html: Option<String>,
    content_text: Option<String>,
    summary: Option<String>,
    image: Option<Url>,
    banner: Option<Url>,
}