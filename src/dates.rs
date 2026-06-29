use chrono::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::DatesBuildError;

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

    pub fn build(&self) -> Result<Dates, DatesBuildError> {
        match (self.published.clone(), self.modified.clone()) {
            (Some(published), Some(modified)) => if let Err(error) = DateTime::parse_from_rfc3339(&published) {
                Err(DatesBuildError::DateParseError(error))
            } else if let Err(error) = DateTime::parse_from_rfc3339(&modified) {
                Err(DatesBuildError::DateParseError(error))
            } else {
                Ok(Dates {
                    published: Some(DateTime::parse_from_rfc3339(&published).unwrap().into()),
                    modified: Some(DateTime::parse_from_rfc3339(&modified).unwrap().into())
                })
            },
            (Some(published), None) => match DateTime::parse_from_rfc3339(&published) {
                Ok(date_published) => Ok(Dates { 
                    published: Some(date_published.into()), 
                    modified: None 
                }),
                Err(error) => Err(DatesBuildError::DateParseError(error))
            },
            (None, Some(modified)) => match DateTime::parse_from_rfc3339(&modified) {
                Ok(date_modified) => Ok(Dates { 
                    published: Some(date_modified.into()), 
                    modified: None 
                }),
                Err(error) => Err(DatesBuildError::DateParseError(error))
            },
            (None, None) => Err(DatesBuildError::NoDates)
        }
    }
}