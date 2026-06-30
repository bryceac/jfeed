use serde::{ Serialize, Deserialize };
use url::Url;
use crate::{FeedVersion, Author, Item, Hub, FeedBuildError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {
    pub version: Url,
    pub title: String,
    #[serde(rename ="home_page_url")]
    pub homepage: Url,
    #[serde(rename = "feed_url")]
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<Url>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "expired_is_default")]
    pub expired: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hubs: Vec<Hub>,
    pub items: Vec<Item>
}

#[derive(Default)]
pub struct FeedBuilder {
    version: Option<FeedVersion>,
    title: Option<String>,
    homepage: Option<String>,
    url: Option<String>,
    description: Option<String>,
    comment: Option<String>,
    next_url: Option<String>,
    icon: Option<String>,
    favicon: Option<String>,
    authors: Vec<Author>,
    language: Option<String>,
    expired: bool,
    hubs: Vec<Hub>,
    items: Vec<Item>
}

impl FeedBuilder {
    pub fn set_version(&mut self, version: &FeedVersion) -> &mut Self {
        self.version = Some(version.clone());
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn set_homepage(&mut self, homepage: &str) -> &mut Self {
        self.homepage = Some(homepage.to_owned());
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn set_comment(&mut self, comment: &str) -> &mut Self {
        self.comment = Some(comment.to_owned());
        self
    }

    pub fn set_next_url(&mut self, next_url: &str) -> &mut Self {
        self.next_url = Some(next_url.to_owned());
        self
    }

    pub fn set_icon(&mut self, icon: &str) -> &mut Self {
        self.icon = Some(icon.to_owned());
        self
    }

    pub fn set_favicon(&mut self, favicon: &str) -> &mut Self {
        self.favicon = Some(favicon.to_owned());
        self
    }

    pub fn add_author(&mut self, author: &Author) -> &mut Self {
        self.authors.push(author.clone());
        self
    }

    pub fn set_language(&mut self, language: &str) -> &mut Self {
        self.language = Some(language.to_owned());
        self
    }

    pub fn set_expired(&mut self, expired: &bool) -> &mut Self {
        self.expired = *expired;
        self
    }

    pub fn add_hub(&mut self, hub: &Hub) -> &mut Self {
        self.hubs.push(hub.clone());
        self
    }

    pub fn add_item(&mut self, item: &Item) -> &mut Self {
        self.items.push(item.clone());
        self
    }

    pub fn build(&self) -> Result<Feed, FeedBuildError> {
        if self.version.is_none()  {
            return Err(FeedBuildError::MissingVersion);
        }

        if self.title.is_none() {
            return  Err(FeedBuildError::MissingTitle);
        }

        if self.homepage.is_none() {
            return Err(FeedBuildError::MissingHomePage);
        }

        if self.url.is_none() {
            return Err(FeedBuildError::MissingURL);
        }

        if self.items.is_empty() {
            return Err(FeedBuildError::MissItems);
        }
    }
}

fn expired_is_default(expired: &bool) -> bool {
    *expired == bool::default()
}