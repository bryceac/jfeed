use std::str::FromStr;

/// Version of JSON feed being targetted
#[derive(Clone, Debug)]
pub enum Version {
    JSONFeed1_1
}

impl ToString for Version {
    /// returns a URL string for the specified JSON feed version.
    fn to_string(&self) -> String {
        match self {
            Self::JSONFeed1_1 => "https://jsonfeed.org/version/1.1".to_owned()
        }
    }
}

impl FromStr for Version {
    type Err = &'static str;

    /// takes a URL string and returns a version.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "https://jsonfeed.org/version/1.1" => Ok(Self::JSONFeed1_1),
            _ => Err("invalid version.")
        }
    }
}