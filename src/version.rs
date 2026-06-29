use std::str::FromStr;

use serde::{ Serialize, Serializer, Deserialize, de::{ self, Visitor } };

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
        match self {
            Self::JSONFeed1_1 => serializer.serialize_str(&Self::JSONFeed1_1.to_string())
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

                deserializer.deserialize_str(VersionVisitor)
        
    }
}

struct VersionVisitor;

impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a URL matching a spec version of JSON Feed")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        match v {
            s if s == &Version::JSONFeed1_1.to_string() => Ok(Version::JSONFeed1_1),
            _ => Err(E::custom("not valid version"))
        }
    }
}