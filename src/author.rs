use std::collections::HashMap;

use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Url>
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct AuthorDes(HashMap<String, Option<String>>);

impl TryFrom<AuthorDes> for Author {
    type Error = &'static str;

    fn try_from(mut value: AuthorDes) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err("no data provided, Please provide either a name, url, or avatar at minimum.")
        }
        match value.0
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
}