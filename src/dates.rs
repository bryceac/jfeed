use std::collections::HashMap;

use chrono::prelude::*;
use serde::{ Serialize, Deserialize, Serializer, ser::SerializeStruct };

use crate::DatesBuildError;

#[derive(Deserialize, Clone, Debug)]
#[serde(try_from = "DatesDes")]
pub struct Dates {
    #[serde(rename = "date_published", skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    #[serde(rename = "date_modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
}

impl Dates {
    pub fn builder() -> DatesBuilder {
        DatesBuilder::default()
    }
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

#[derive(Deserialize)]
#[serde(transparent)]
struct DatesDes(HashMap<String, String>);

impl TryFrom<DatesDes> for Dates {
    type Error = DatesBuildError;

    fn try_from(value: DatesDes) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(DatesBuildError::NoDates);
        }

        let mut builder = DatesBuilder::default();

        for key in value.0.keys() {
            match key {
                s if s == "date_published" => if let Some(date_published) = value.0.get(key) {
                    builder.set_published(&date_published);
                },
                s if s == "date_modified" => if let Some(date_modified) = value.0.get(key) {
                    builder.set_modified(&date_modified);
                },
                _ => {}
            }
        }

        builder.build()
    }
}

impl Serialize for Dates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            let mut state = serializer.serialize_struct("Dates", 2)?;
            
            if let Some(date_published) = self.published {
                state.serialize_field("date_published", &date_published.to_rfc3339())?;
            }

            if let Some(date_modified) = self.modified {
                state.serialize_field("date_published", &date_modified.to_rfc3339())?;
            }

            state.end()
        }
}