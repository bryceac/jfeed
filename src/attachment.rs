use std::error::Error;

use serde::{ Serialize, Deserialize };
use url::Url;

use crate::AttachmentBuildError;

/**
 * JSON Feed attachment, 
 * which is typically used for delivering podcasts.
 * 
 * For more information, refer to the following page for more details.
 * 
 * https://www.jsonfeed.org/version/1.1/index.html#attachments
 */

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    /// The attachment's URL
    pub url: Url,
    /// The mimetype for the attached item
    pub mime_type: String,
    /// Title title for the attached item
    #[serde(default = "String::default", skip_serializing_if = "String::is_empty")]
    pub title: String,
    /// the size of the attachment in bytes
    #[serde(rename = "size_in_bytes", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// the duration of the attached item in seconds
    #[serde(rename = "duration_in_seconds", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>
}

impl Attachment {
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder::default()
    }
}

/**
 * A convenience building type, to make it easy to create an attacment.
 */

#[derive(Default)]
pub struct AttachmentBuilder {
    url: Option<String>,
    mime_type: Option<String>,
    title: Option<String>,
    size: Option<u32>,
    duration: Option<u32>,
}

impl AttachmentBuilder {
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn set_mimetype(&mut self, mimetype: &str) -> &mut Self {
        self.mime_type = Some(mimetype.to_owned());
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn set_size(&mut self, size_in_bytes: u32) -> &mut Self {
        self.size = Some(size_in_bytes);
        self
    }

    pub fn set_duration(&mut self, duration_in_seconds: u32) -> &mut Self {
        self.duration = Some(duration_in_seconds);
        self
    }

    /**
     * Build an attachment.
     * If any errors occur, an AttachmentBuldError will be returned.
     */
    pub fn build(&self) -> Result<Attachment, AttachmentBuildError> {
        match (self.url.clone(), self.mime_type.clone()) {
            (None, Some(_)) => Err(AttachmentBuildError::URLNotFound),
            (Some(_), None) => Err(AttachmentBuildError::MimetypeNotFound),
            (None, None) => Err(AttachmentBuildError::URLAndMimetypeNotFound),
            (Some(url), Some(mime_type)) => match Url::parse(&url) {
                Ok(parsed_url) => Ok(Attachment {
                    url: parsed_url,
                    mime_type,
                    title: if let Some(title) = self.title.clone() {
                        title
                    } else {
                        String::default()
                    },
                    size: self.size,
                    duration: self.duration
                }),
                Err(error) => Err(AttachmentBuildError::URLParseError(error))
            }
        }
    }
}