use http::uri::Uri;
use serde::{ Serializer, Deserializer };

pub fn serialize<S>(uri: &Option<Uri>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
    if let Some(uri) = uri {
        http_serde::uri::serialize(uri, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uri>, D::Error> where D: Deserializer<'de>, {
    let uri = http_serde::uri::deserialize(deserializer)?;
    Ok(Some(uri))
}