use std::{ error::Error, fmt };
use url::ParseError;

#[derive(Debug)]
pub enum AttachmentBuildError {
    URLNotFound,
    MimetypeNotFound,
    URLAndMimetypeNotFound,
    URLParseError(ParseError)
}

impl fmt::Display for AttachmentBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MimetypeNotFound => write!(f, "mimetype must be specified."),
            Self::URLNotFound => write!(f, "URL must be specified."),
            Self::URLAndMimetypeNotFound => write!(f, "Both URL and mimetype must be specified"),
            Self::URLParseError(parseError) => write!(f, "{}", parseError)
        }
    }
}

pub enum AuthorBuildError {
    MissingData,
    URLParseError(ParseError)
}

impl fmt::Display for AuthorBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => write!(f, "No data found. Please provide either a name, URL, or avatar URL."),
            Self::URLParseError(parseError) => write!(f, "{}", parseError)
        }
    }
}