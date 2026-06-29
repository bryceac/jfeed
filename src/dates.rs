use chrono::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dates {
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<DateTime<Utc>>,
}