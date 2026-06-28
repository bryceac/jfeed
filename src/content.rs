use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::ContentBuildError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(rename = "content_html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "content_text", skip_serializing_if ="Option::is_none")]
    pub text: Option<String>
}

#[derive(Default)]
pub struct ContentBuilder {
    html: Option<String>,
    text: Option<String>
}

impl ContentBuilder {
    pub fn set_html(&mut self,html: &str) -> &mut Self {
        self.html = Some(html.to_owned());
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = Some(text.to_owned());
        self
    }

    pub fn build(&self) -> Result<Content, ContentBuildError> {
        Content { html: (), text: () }
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ContentDes(HashMap<String, String>);

impl TryFrom<ContentDes> for Content {
    type Error = ContentBuildError;

    fn try_from(mut value: ContentDes) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(ContentBuildError::MissingContent);
        }

        match value.0.keys {

        }
    }
}