mod attachment;
mod author;
mod item;
mod content;
mod dates;
mod errors;

use crate::{
    attachment::Attachment as Attachment, 
    errors::AttachmentBuildError as AttachmentBuildError,
    errors::AuthorBuildError as AuthorBuildError,
    errors::ContentBuildError as ContentBuildError,
    errors::DatesBuildError as DatesBuildError,
    errors::ItemBuildError as ItemBuildError,
    author::Author as Author,
    content::Content as Content,
    dates::Dates as Dates 
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
    fn author_building_fails_with_no_data() {
        assert!(Author::builder()
        .build().is_err())
    }

    #[test]
    fn author_building_fails_with_invalid_url() {
        assert!(Author::builder()
        .set_url("mp4").build().is_err())
    }

    #[test]
    fn author_building_fails_with_invalid_avatar() {
        assert!(Author::builder()
        .set_avatar("mp4")
        .build().is_err())
    }

    #[test]
    fn author_building_fails_with_name_but_invalid_url() {
        assert!(Author::builder()
        .set_name("Jerry")
        .set_url("mp4")
        .build().is_err())
    }

    #[test]
    fn author_building_fails_with_name_but_invalid_avatar() {
        assert!(Author::builder()
        .set_name("Jerry")
        .set_avatar("mp4")
        .build().is_err())
    }

    #[test]
    fn author_building_fails_with_name_but_invalid_urls() {
        assert!(Author::builder()
        .set_name("Jerry")
        .set_url("html")
        .set_avatar("mp4")
        .build().is_err())
    }

    #[test]
    fn create_author_with_only_name() {
        assert!(Author::builder()
        .set_name("Jerry")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_url() {
        assert!(Author::builder()
        .set_url("https://example.com")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_avatar() {
        assert!(Author::builder()
        .set_avatar("https://example.com/jerry.png")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_url_and_avatar() {
        assert!(Author::builder()
        .set_url("https://example.com")
        .set_avatar("https://example.com/jerry.png")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_url_and_name() {
        assert!(Author::builder()
        .set_url("https://example.com")
        .set_name("Jerry")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_name_and_avatar() {
        assert!(Author::builder()
        .set_name("Jerry")
        .set_avatar("https://example.com/jerry.png")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_all_properties() {
        let mut builder = Author::builder();
        builder.set_name("Jerry");
        builder.set_url("https://example.com");
        builder.set_avatar("https://example.com/jerry.png");

    assert!(builder.build().is_ok())
    }

    #[test]
    fn content_building_fails_with_no_content() {
        assert!(Content::builder().build().is_err())
    }

    #[test]
    fn content_building_works_with_only_html() {
        assert!(Content::builder().set_html("<p>Hello, World</p>").build().is_ok())
    }

    #[test]
    fn content_building_works_with_only_text() {
        assert!(Content::builder().set_text("Hello, World!").build().is_ok())
    }

    #[test]
    fn can_build_content_with_both_plain_and_html() {
        assert!(Content::builder()
        .set_html("<p>Hello, World!</p>")
        .set_text("Hello, World!")
        .build().is_ok())
    }

    #[test]
    fn dates_build_fails_without_dates() {
        assert!(Dates::builder().build().is_err())
    }

    #[test]
    fn dates_build_works_with_only_published() {
        let date = "2026-06-28T08:08:00Z";
        assert!(Dates::builder().set_published(date).build().is_ok())
    }

    #[test]
    fn dates_build_works_with_only_modified() {
        let date = "2026-06-28T08:08:00Z";
        assert!(Dates::builder().set_modified(date).build().is_ok())
    }

    #[test]
    fn dates_build_works_with_both_dates() {
        let published = "2026-06-28T08:08:00Z";
        let modified = "2026-06-28T08:44:00Z";
        assert!(Dates::builder()
        .set_published(published)
        .set_modified(modified).build().is_ok())
    }
}
