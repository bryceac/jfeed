use serde::{ Deserialize, Serialize };
use url::Url;

use crate::HubError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hub {
    #[serde(rename = "type")]
    pub hub_type: String,
    pub url: Url
}

impl Hub {
    pub fn from(hub_type: &str, url: &str) -> Result<Self, HubError> {
        match (hub_type, url) {
            (t, u) if t.is_empty() && u.is_empty() => Err(HubError::MissingAll),
            (t, u) if !t.is_empty() && u.is_empty() => Err(HubError::NoURL),
            (t, u) if t.is_empty() && !u.is_empty() => Err(HubError::NoType),
            _ => match Url::parse(url) {
                Ok(parsed_url) => Ok(Hub {
                    hub_type: hub_type.to_owned(),
                    url: parsed_url
                }),
                Err(error) => Err(HubError::URLError(error))
            }
        }
    }
}

impl PartialEq for Hub {
    fn eq(&self, other: &Self) -> bool {
        self.hub_type == other.hub_type &&
        self.url == other.url
    }
}