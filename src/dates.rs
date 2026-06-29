use chrono::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dates {
    #[serde(rename = "date_published", skip_serializing_if = "Option::is_none")]
    published: Option<DateTime<Utc>>,
    #[serde(rename = "date_modified", skip_serializing_if = "Option::is_none")]
    modified: Option<DateTime<Utc>>,
}