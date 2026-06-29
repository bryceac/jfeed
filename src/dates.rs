use chrono::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dates {
    #[serde(rename = "date_published", skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    #[serde(rename = "date_modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct DatesBuilder {
    published: Option<String>,
    modified: Option<String>
}

impl DatesBuilder {
    pub fn set_published(&mut self, published: &str) -> &mut Self {
        self.published = Some(published.to_owned());
        self
    }

    pub fn set_modified(&mut self, modified: &str) -> &mut Self {
        self.modified = Some(modified.to_owned());
        self
    }

    pub fn build(&self) -> Dates {
        
    }
}