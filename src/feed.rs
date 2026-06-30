use serde::{ Serialize, Deserialize };
use serde_json::Result as JSONResult;
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
    #[serde(rename = "user_comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Url>,
    #[serde(rename = "favicon", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<Url>,
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

impl Feed {
    pub fn builder() -> FeedBuilder {
        FeedBuilder::default()
    }

    pub fn from_str(text: &str) -> JSONResult<Self> {
        serde_json::from_str(&text)
    }

    pub fn to_string(&self) -> JSONResult<String> {
        serde_json::to_string_pretty(&self)
    }
}

impl PartialEq for Feed {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version &&
        self.title == other.title &&
        self.homepage == other.homepage &&
        self.url == other.url &&
        self.description == other.description &&
        self.comment == other.comment &&
        self.next_url == other.next_url &&
        self.icon_url == other.icon_url &&
        self.favicon_url == other.favicon_url &&
        self.authors == other.authors &&
        self.language == other.language &&
        self.expired == other.expired &&
        self.hubs == other.hubs &&
        self.items == other.items
    }
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
    icon_url: Option<String>,
    favicon_url: Option<String>,
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

    pub fn set_icon_url(&mut self, icon_url: &str) -> &mut Self {
        self.icon_url = Some(icon_url.to_owned());
        self
    }

    pub fn set_favicon_url(&mut self, favicon_url: &str) -> &mut Self {
        self.favicon_url = Some(favicon_url.to_owned());
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

        match Url::parse(&self.version.clone().unwrap().to_string()) {
            Ok(parsed_url) => {
                if let Some(homepage) = self.homepage.clone() {
                    if let Err(error) = Url::parse(&homepage) {
                        return Err(FeedBuildError::URLError(error))
                    }
                }

                if let Some(feed_url) = self.url.clone() {
                    if let Err(error) = Url::parse(&feed_url) {
                        return Err(FeedBuildError::URLError(error))
                    }
                }

                if let Some(next_url) = self.next_url.clone() {
                    if let Err(error) = Url::parse(&next_url) {
                        return Err(FeedBuildError::URLError(error))
                    }
                }

                if let Some(icon_url) = self.icon_url.clone() {
                    if let Err(error) = Url::parse(&icon_url) {
                        return Err(FeedBuildError::URLError(error))
                    }
                }

                if let Some(favicon_url) = self.favicon_url.clone() {
                    if let Err(error) = Url::parse(&favicon_url) {
                        return Err(FeedBuildError::URLError(error))
                    }
                }

                Ok(Feed { 
                    version: parsed_url.clone(), 
                    title: self.title.clone().unwrap(), 
                    homepage: Url::parse(&self.homepage.clone().unwrap()).unwrap(), 
                    url: Url::parse(&self.url.clone().unwrap()).unwrap(), 
                    description: self.description.clone(), 
                    comment: self.comment.clone(), 
                    next_url: if let Some(next_url) = self.next_url.clone() {
                        Some(Url::parse(&next_url).unwrap())
                    } else {
                        None
                    }, 
                    icon_url: if let Some(icon_url) = self.icon_url.clone() {
                        Some(Url::parse(&icon_url).unwrap())
                    } else {
                        None
                    }, 
                    favicon_url: if let Some(favicon_url) = self.favicon_url.clone() {
                        Some(Url::parse(&favicon_url).unwrap())
                    } else {
                        None
                    }, 
                    authors: self.authors.clone(), 
                    language: self.language.clone(), 
                    expired: self.expired, 
                    hubs: self.hubs.clone(), 
                    items: self.items.clone() 
                })
            },
            Err(error) => Err(FeedBuildError::URLError(error))
        }
    }
}

fn expired_is_default(expired: &bool) -> bool {
    *expired == bool::default()
}