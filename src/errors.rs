use std::{ error::Error, fmt };

#[derive(Debug)]
pub enum AttachmentBuildError {
    URLNotFound,
    MimeTypeNotFound
}

impl fmt::Display for AttachmentBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::MimeTypeNotFound => write!(f, "mimetype must be specified."),
            Self::URLNotFound => write!(f, "URL must be specified.")
        }
    }
}