mod attachment;
mod author;
mod item;
mod errors;

use crate::{
    attachment::Attachment as Attachment, 
    errors::AttachmentBuildError as AttachmentBuildError,
    errors::AuthorBuildError as AuthorBuildError,
    author::Author as Author 
};

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn attachment_build_error_displays_correctly() {
        assert_eq!(format!("{}", AttachmentBuildError::URLNotFound), "URL must be specified.");
    }

    #[test]
    fn attachment_build_fails_when_url_is_missing() {
        let mut attachment = Attachment::builder();
        attachment.set_mimetype("video/mp4");
        attachment.set_title("Hello, World!");

        assert!(attachment.build().is_err())
    }

    #[test]
    fn attachment_build_fails_when_mimtetype_is_missing() {
        let mut attachment = Attachment::builder();
        attachment.set_url("https://example.com/hello.mp4");
        attachment.set_title("Hello, World!");

        assert!(attachment.build().is_err())
    }

    #[test]
    fn attachment_build_fails_when_all_required_is_missing() {
        let mut attachment = Attachment::builder();
        attachment.set_title("Hello, World!");

        assert!(attachment.build().is_err())
    }

    #[test]
    fn attachment_build_fails_when_url_cannot_be_parsed() {
        let mut attachment = Attachment::builder();
        attachment.set_url("mp4");
        attachment.set_mimetype("video/mp4");
        attachment.set_title("Hello, World!");

        assert!(attachment.build().is_err())
    }

    #[test]
    fn attachment_builds_successfully() {
        let mut attachment = Attachment::builder();
        attachment.set_url("https://example.co/hello.mp4");
        attachment.set_mimetype("video/mp4");
        attachment.set_title("Hello, World!");

        assert!(attachment.build().is_ok())
    }

    #[test]
}
