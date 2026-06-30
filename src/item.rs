use serde::{ Serialize, Deserialize };
use url::Url;
use crate::{ Author, Content, Dates, Attachment, ItemBuildError };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    id: String,
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(flatten)]
    content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<Url>,
    #[serde(flatten)]
    dates: Dates,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment>
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::default()
    }
}

#[derive(Default)]
pub struct ItemBuilder {
    id: Option<String>,
    url: Option<String>,
    external_url: Option<String>,
    title: Option<String>,
    content: Option<Content>,
    summary: Option<String>,
    image: Option<String>,
    banner: Option<String>,
    dates: Option<Dates>,
    authors: Vec<Author>,
    tags: Vec<String>,
    language: Option<String>,
    attachments: Vec<Attachment>
}

impl ItemBuilder {
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_owned());
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn set_external_url(&mut self, url: &str) -> &mut Self {
        self.external_url = Some(url.to_owned());
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn set_content(&mut self, content: &Content) -> &mut Self {
        self.content = Some(content.clone());
        self
    }

    pub fn set_summary(&mut self, summary: &str) -> &mut Self {
        self.summary = Some(summary.to_owned());
        self
    }

    pub fn set_image_url(&mut self, image: &str) -> &mut Self {
        self.image = Some(image.to_owned());
        self
    }

    pub fn set_banner(&mut self, banner: &str) -> &mut Self {
        self.banner = Some(banner.to_owned());
        self
    }

    pub fn set_dates(&mut self, dates: &Dates) -> &mut Self {
        self.dates = Some(dates.clone());
        self
    }

    pub fn set_language(&mut self, lang: &str) -> &mut Self {
        self.language = Some(lang.to_owned());
        self
    }

    pub fn add_tag(&mut self, tag: &str) -> &mut Self {
        self.tags.push(tag.to_owned());
        self
    }

    pub fn add_author(&mut self, author: &Author) -> &mut Self {
        self.authors.push(author.to_owned());
        self
    }

    pub fn add_attachment(&mut self, attachment: &Attachment) -> &mut Self {
        self.attachments.push(attachment.to_owned());
        self
    }

    pub fn build(&self) -> Result<Item, ItemBuildError> {
        if self.id.is_none() {
            return Err(ItemBuildError::IDNotFound);
        }

        if self.url.is_none() {
            return Err(ItemBuildError::URLNotFound);
        }

        if self.authors.is_empty() {
            return Err(ItemBuildError::NoAuthorsFound);
        }

        if self.content.is_none() {
            return Err(ItemBuildError::NoContent);
        }

        if self.dates.is_none() {
            return Err(ItemBuildError::NoDate);
        }

        match Url::parse(&self.url.clone().unwrap()) {
            Ok(parsed_url) => {
                if let Some(external_url) = self.external_url.clone() {
                    if let Err(parse_error) = Url::parse(&external_url) {
                        return Err(ItemBuildError::MiscError(parse_error));
                    }
                }

                if let Some(image) = self.image.clone() {
                    if let Err(parse_error) = Url::parse(&image) {
                        return Err(ItemBuildError::MiscError(parse_error));
                    }
                }

                if let Some(banner) = self.banner.clone() {
                    if let Err(parse_error) = Url::parse(&banner) {
                        return Err(ItemBuildError::MiscError(parse_error));
                    }
                }

                Ok(Item {
                    id: self.id.clone().unwrap(),
                    url: parsed_url.clone(),
                    external_url: if let Some(external_url) = self.external_url.clone() {
                        Some(Url::parse(&external_url).unwrap())
                    } else {
                        None
                    },
                    title: self.title.clone(),
                    content: self.content.clone().unwrap(),
                    summary: self.summary.clone(),
                    image: if let Some(image) = self.image.clone() {
                        if let Ok(parsed_image_url) = Url::parse(&image) {
                            Some(parsed_image_url)
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    banner: if let Some(banner) = self.banner.clone() {
                        if let Ok(parsed_banner_url) = Url::parse(&banner) {
                            Some(parsed_banner_url)
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                    dates: self.dates.clone().unwrap(),
                    authors: self.authors.clone(),
                    tags: self.tags.clone(),
                    language: self.language.clone(),
                    attachments: self.attachments.clone()
                })
            },
            Err(parse_error) => Err(ItemBuildError::MiscError(parse_error))
        }
    }
}