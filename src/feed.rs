use std::str::FromStr;

use serde::{ Serialize, Deserialize };
use url::Url;

#[derive(Clone, Debug)]
pub enum Version {
    JSONFeed1_1
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::JSONFeed1_1 => "https://jsonfeed.org/version/1.1".to_owned()
        }
    }
}

impl FromStr for Version {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "https://jsonfeed.org/version/1.1" => Ok(Self::JSONFeed1_1),
            _ => Err("invalid version.")
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {
    pub version: Version
}