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
pub enum ItemBuildError<T: Error> {
    IDNotFound,
    URLNotFound,
    NoAuthorsFound,
    MiscError(T)
}

impl<T: Error> fmt::Display for ItemBuildError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::IDNotFound => write!(f, "Please provide an identifier."),
            Self::URLNotFound => write!(f, "Please provide a URL"),
            Self::NoAuthorsFound => write!(f, "Item must have at least one author."),
            Self::MiscError(error) => write!(f, "{}", error)
        }
    }
}

impl<T: Error> Error for ItemBuildError<T> {}