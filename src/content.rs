use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::ContentBuildError;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "ContentDes")]
pub struct Content {
    #[serde(rename = "content_html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "content_text", skip_serializing_if ="Option::is_none")]
    pub text: Option<String>
}

impl Content {
    pub fn builder() -> ContentBuilder {
        ContentBuilder::default()
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.html == other.html &&
        self.text == other.text
    }
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
        match (self.html.clone(), self.text.clone()) {
            (Some(html), Some(text)) => Ok(Content {
                html: Some(html),
                text: Some(text)
            }),
            (None, Some(text)) => Ok(Content {
                html: None,
                text: Some(text)
            }),
            (Some(html), None) => Ok(Content {
                html: Some(html),
                text: None
            }),
            (None, None) => Err(ContentBuildError::MissingContent)
        }
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ContentDes(HashMap<String, String>);

impl TryFrom<ContentDes> for Content {
    type Error = ContentBuildError;

    fn try_from(value: ContentDes) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(ContentBuildError::MissingContent);
        }

        let mut builder = ContentBuilder::default();

        for key in value.0.keys() {
            match key {
                s if s == "content_html" => if let Some(html) = value.0.get(key) {
                    builder.set_html(html);
                },
                s if s == "content_text" => if let Some(text) = value.0.get(key) {
                    builder.set_text(text);
                }
                _ => {}
            }
        }

        builder.build()
    }
}