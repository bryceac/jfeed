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
    #[serde(rename = "size_in_bytes", skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>
}

/**
 * A convenience building type, to make it easy to create an attacment.
 */
pub struct AttachmentBuilder {
    url: Option<String>,
    mime_type: Option<String>,
    title: Option<String>,
    size: Option<u32>,
    duration: Option<u32>,
}