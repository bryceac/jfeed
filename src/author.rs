use std::collections::HashMap;

use serde::{ Serialize, Deserialize };
use url::Url;

use crate::AuthorBuildError;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Url>
}

impl Author {
    pub fn builder() -> AuthorBuilder {
        AuthorBuilder::default()
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct AuthorDes(HashMap<String, String>);

impl TryFrom<AuthorDes> for Author {
    type Error = AuthorBuildError;

    fn try_from(mut value: AuthorDes) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(AuthorBuildError::MissingData)
        }

        let mut builder = Author::builder();

        for key in value.0.keys() {
            match key.clone() {
                s if s == "name" => if let Some(name) = value.remove(key) {
                    builder.set_name(name);
                },
                s if s == "url" => if let Some(url) = value.remove(key) {
                    builder.set_url(url);
                },
                s if s == "avatar" => if let Some(avatar_url) = value.remove(key) {
                    builder.set_avatar(avatar_url);
                },
                _ => {}
            }
        }

        builder.build()
    }
}

#[derive(Default)]
pub struct AuthorBuilder {
    name: Option<String>,
    url: Option<String>,
    avatar: Option<String>
}

impl AuthorBuilder {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn set_avatar(&mut self, avatar: &str) -> &mut Self {
        self.avatar = Some(avatar.to_owned());
        self
    }

    pub fn build(&self) -> Result<Author, AuthorBuildError> {
        match (self.name.clone(), self.url.clone(), self.avatar.clone()) {
            (None, None, None) => Err(AuthorBuildError::MissingData),
            (Some(name), Some(url), Some(avatar)) => {
                if let Err(parse_error) = Url::parse(&url) {
                    Err(AuthorBuildError::URLParseError(parse_error))
                } else if let Err(parse_error) = Url::parse(&avatar) {
                    Err(AuthorBuildError::URLParseError(parse_error))
                } else {
                    Ok(Author {
                        name: Some(name),
                        url: Some(Url::parse(&url).unwrap()),
                        avatar: Some(Url::parse(&avatar).unwrap()),
                    })
                }
            },
            (Some(name), Some(url), None) => match Url::parse(&url) {
                Ok(author_url) => Ok(Author {
                    name: Some(name),
                    url: Some(author_url),
                    avatar: None
                }),
                Err(parse_error) => Err(AuthorBuildError::URLParseError(parse_error))
            },
            (Some(name), None, Some(avatar)) => match Url::parse(&avatar) {
                Ok(avatar_url) => Ok(Author {
                    name: Some(name),
                    url: None,
                    avatar: Some(avatar_url)
                }),
                Err(parse_error) => Err(AuthorBuildError::URLParseError(parse_error))
            },
            (None, Some(url), Some(avatar)) => if let Err(parse_error) = Url::parse(&url) {
                Err(AuthorBuildError::URLParseError(parse_error))
            } else if let Err(parse_error) = Url::parse(&avatar) {
                Err(AuthorBuildError::URLParseError(parse_error))
            } else {
                Ok(Author {
                    name: None,
                    url: Some(Url::parse(&url).unwrap()),
                    avatar: Some(Url::parse(&avatar).unwrap())
                })
            },
            (Some(name), None, None) => Ok(Author { 
                name: Some(name), 
                url: None, 
                avatar: None 
            }),
            (None, Some(url), None) => match Url::parse(&url) {
                Ok(author_url) => Ok(Author { 
                    name: None, 
                    url: Some(author_url), 
                    avatar: None 
                }),
                Err(parse_error) => Err(AuthorBuildError::URLParseError(parse_error))
            },
            (None, None, Some(avatar)) => match Url::parse(&avatar) {
                Ok(avatar_url) => Ok(Author { 
                    name: None, 
                    url: None, 
                    avatar: Some(avatar_url) 
                }),
                Err(parse_error) => Err(AuthorBuildError::URLParseError(parse_error))
            }
        }
    }
}