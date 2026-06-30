use core::num::dec2flt::parse;
use std::{ error::Error, fmt };
use url::ParseError as URLParseError;
use chrono::format::ParseError as ChronoParseError;

#[derive(Debug)]
pub enum AttachmentBuildError {
    URLNotFound,
    MimetypeNotFound,
    URLAndMimetypeNotFound,
    URLParseError(URLParseError)
}

impl fmt::Display for AttachmentBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MimetypeNotFound => write!(f, "mimetype must be specified."),
            Self::URLNotFound => write!(f, "URL must be specified."),
            Self::URLAndMimetypeNotFound => write!(f, "Both URL and mimetype must be specified"),
            Self::URLParseError(parse_error) => write!(f, "{}", parse_error)
        }
    }
}

impl Error for AttachmentBuildError {}

#[derive(Debug)]
pub enum AuthorBuildError {
    MissingData,
    URLParseError(URLParseError)
}

impl fmt::Display for AuthorBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingData => write!(f, "No data found. Please provide either a name, URL, or avatar URL."),
            Self::URLParseError(parse_error) => write!(f, "{}", parse_error)
        }
    }
}

impl Error for AuthorBuildError {}

#[derive(Debug)]
pub enum ContentBuildError {
    MissingContent
}

impl fmt::Display for ContentBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingContent => write!(f, "No data found. Please provide some content."),
        }
    }
}

impl Error for ContentBuildError {}

#[derive(Debug)]
pub enum DatesBuildError {
    DateParseError(ChronoParseError),
    NoDates,
}

impl fmt::Display for DatesBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoDates => write!(f, "No data found. Please provide either a publication or modification date."),
            Self::DateParseError(parse_error) => write!(f, "{}", parse_error)
        }
    }
}

impl Error for DatesBuildError {}

#[derive(Debug)]
pub enum ItemBuildError {
    IDNotFound,
    URLNotFound,
    NoAuthorsFound,
    NoContent,
    NoDate,
    MiscError(URLParseError)
}

impl fmt::Display for ItemBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::IDNotFound => write!(f, "Please provide an identifier."),
            Self::URLNotFound => write!(f, "Please provide a URL"),
            Self::NoAuthorsFound => write!(f, "Item must have at least one author."),
            Self::NoContent => write!(f, "Please provide content in plain text or HTML."),
            Self::NoDate => write!(f, "Please provide a publication and/or modification date."),
            Self::MiscError(parse_error) => write!(f, "{}", parse_error)
        }
    }
}

impl Error for ItemBuildError {}

#[derive(Debug)]
pub enum HubError {
    NoType,
    NoURL,
    MissingAll,
    URLError(URLParseError)
}

impl fmt::Display for HubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingAll => write!(f, "Hubs must provide a type and a URL"),
            Self::NoURL => write!(f, "Hubs must have a URL."),
            Self::NoType => write!(f, "Hubs must have a type."),
            Self::URLError(parse_error) => write!(f, "{}", parse_error)
        }
    }
}

impl Error for HubError {}