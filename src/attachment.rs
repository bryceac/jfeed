use serde::{ Serialize, Deserialize };
use url::Url;

/**
 * JSON Feed attachment, 
 * which is typically used for delivering podcasts.
 * 
 * For more information, refer to the following page for more details.
 * 
 * https://www.jsonfeed.org/version/1.1/index.html#attachments
 */

#[derive(Serialize, Deserialize, Clone)]
pub struct Attachment {
    url: Url,
    mime_type: String,
    #[serde(default = "String::default", skip_serializing_if = "String::is_empty")]
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>
}