mod attachment;
mod errors;

use crate::{
    attachment::Attachment as Attachment, 
    errors::AttachmentBuildError as AttachmentBuildError 
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
}
