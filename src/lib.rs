mod attachment;
mod author;
mod item;
mod content;
mod dates;
mod hub;
mod feed;
mod version;
mod errors;

pub use crate::{
    attachment::Attachment as Attachment, 
    errors::AttachmentBuildError as AttachmentBuildError,
    errors::AuthorBuildError as AuthorBuildError,
    errors::ContentBuildError as ContentBuildError,
    errors::DatesBuildError as DatesBuildError,
    errors::ItemBuildError as ItemBuildError,
    errors::FeedBuildError as FeedBuildError,
    errors::HubError as HubError,
    author::Author as Author,
    content::Content as Content,
    dates::Dates as Dates,
    item::Item as Item,
    feed::Feed as Feed,
    hub::Hub as Hub,
    version::Version as FeedVersion
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
        .set_avatar_url("mp4")
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
        .set_avatar_url("mp4")
        .build().is_err())
    }

    #[test]
    fn author_building_fails_with_name_but_invalid_urls() {
        assert!(Author::builder()
        .set_name("Jerry")
        .set_url("html")
        .set_avatar_url("mp4")
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
        .set_avatar_url("https://example.com/jerry.png")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_only_url_and_avatar() {
        assert!(Author::builder()
        .set_url("https://example.com")
        .set_avatar_url("https://example.com/jerry.png")
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
        .set_avatar_url("https://example.com/jerry.png")
        .build().is_ok())
    }

    #[test]
    fn create_author_with_all_properties() {
        let mut builder = Author::builder();
        builder.set_name("Jerry");
        builder.set_url("https://example.com");
        builder.set_avatar_url("https://example.com/jerry.png");

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

    #[test]
    fn item_build_fails_without_data() {
        let builder = Item::builder();

        assert!(builder.build().is_err())
    }

    #[test]
    fn item_build_fails_without_id() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_content(&content);

        assert_eq!(format!("{}", builder.build().err().unwrap()), format!("{}", ItemBuildError::IDNotFound))
    }

    #[test]
    fn item_build_fails_without_url() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_content(&content);

        assert_eq!(format!("{}", builder.build().err().unwrap()), format!("{}", ItemBuildError::URLNotFound))
    }

    #[test]
    fn item_build_fails_without_author() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        /* let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap(); */

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.set_content(&content);

        assert_eq!(format!("{}", builder.build().err().unwrap()), format!("{}", ItemBuildError::NoAuthorsFound))
    }

    #[test]
    fn item_build_fails_without_dates() {
       /* let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap(); */

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.add_author(&author);
        builder.set_content(&content);

        assert_eq!(format!("{}", builder.build().err().unwrap()), format!("{}", ItemBuildError::NoDate))
    }

    #[test]
    fn item_build_fails_without_content() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        /* let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap(); */

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        // builder.set_content(&content);

        assert_eq!(format!("{}", builder.build().err().unwrap()), format!("{}", ItemBuildError::NoContent))
    }

    fn item_build_fails_with_invalid_main_url() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("hello_world.html");
        builder.set_url("hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_content(&content);

        assert!(builder.build().is_err())
    }

    fn item_build_fails_with_invalid_external_url() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_external_url("image.png");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_content(&content);

        assert!(builder.build().is_err())
    }

    fn item_build_fails_with_invalid_image_url() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_image_url("image.png");
        builder.set_content(&content);

        assert!(builder.build().is_err())
    }

    fn item_build_fails_with_invalid_banner_url() {
        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let mut builder = Item::builder();
        builder.set_id("https://example.com/hello_world.html");
        builder.set_url("https://example.com/hello_world.html");
        builder.set_title("Hello, World!");
        builder.set_banner_url("image.png");
        builder.set_dates(&dates);
        builder.add_author(&author);
        builder.set_content(&content);

        assert!(builder.build().is_err())
    }

    #[test]
    fn item_build_fails_with_any_invalid_urls() {
        item_build_fails_with_invalid_main_url();
        item_build_fails_with_invalid_external_url();
        item_build_fails_with_invalid_banner_url();
        item_build_fails_with_invalid_image_url();
    }

    fn hub_initialization_fails_if_type_is_empty() {
        assert!(Hub::from("", "https://example.com").is_err())
    }

    fn hub_initialization_fails_if_url_is_empty() {
        assert!(Hub::from("Hello", "").is_err())
    }

    fn hub_initialize_fails_when_both_items_are_empty() {
        assert!(Hub::from("", "").is_err())
    }

    fn hub_initialization_fails_with_invalid_url() {
        assert!(Hub::from("Hello", "html").is_err())
    }

    #[test]
    fn hub_initialization_fails_properly() {
        hub_initialization_fails_if_type_is_empty();
        hub_initialization_fails_if_url_is_empty();
        hub_initialize_fails_when_both_items_are_empty();
        hub_initialization_fails_with_invalid_url();
    }

    #[test]
    fn hub_can_be_initialized() {
        assert!(Hub::from("Hello", "https://example.com/hello").is_ok())
    }

    fn feed_building_fails_without_items() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com");
        builder.set_url("https://example.com/feed.json");

        assert!(builder.build().is_err())
    }

    fn feed_building_fails_without_version() {
        let mut builder = Feed::builder();
        builder.set_title("News");
        builder.set_homepage("https://example.com");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_without_url() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_without_homepage() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_without_title() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_with_invalid_url() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_with_invalid_next_url() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");
        builder.set_next_url("feed2.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_with_invalid_icon_url() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");
        builder.set_icon_url("hello.png");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_with_invalid_favicon_url() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");
        builder.set_favicon_url("hello.png");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_banner_url("https://example.com/image.png")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_err())
    }

    fn feed_building_fails_with_invalid_urls() {
        feed_building_fails_with_invalid_url();
        feed_building_fails_with_invalid_next_url();
        feed_building_fails_with_invalid_icon_url();
        feed_building_fails_with_invalid_favicon_url()
    }

    #[test]
    fn feed_building_fails_properly() {
        feed_building_fails_without_version();
        feed_building_fails_without_url();
        feed_building_fails_without_title();
        feed_building_fails_without_homepage();
        feed_building_fails_without_items();
        feed_building_fails_with_invalid_urls();
    }

    #[test]
    fn can_build_feed() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();

        assert!(builder.add_item(&item).build().is_ok())
    }

    #[test]
    fn feed_converts_to_string() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();
        builder.add_item(&item);
        let feed = builder.build().unwrap();

        assert!(feed.to_string().is_ok())
    }

    #[test]
    fn can_parse_feed() {
        let mut builder = Feed::builder();
        builder.set_version(&FeedVersion::JSONFeed1_1);
        builder.set_title("News");
        builder.set_homepage("https://example.com/");
        builder.set_url("https://example.com/feed.json");

        let dates = Dates::builder()
        .set_published("2026-06-28T08:55:00Z")
        .build().unwrap();

        let author = Author::builder()
        .set_name("Jerry")
        .build().unwrap();

        let content = Content::builder()
        .set_text("Hello, World!")
        .build().unwrap();

        let item = Item::builder()
        .set_id("https://example.com/hello_world.html")
        .set_url("https://example.com/hello_world.html")
        .set_title("Hello, World!")
        .set_dates(&dates)
        .add_author(&author)
        .set_content(&content)
        .build().unwrap();
        builder.add_item(&item);
        let expected_feed = builder.build().unwrap();

        let feed = "
        {
            \"version:\" \"https://jsonfeed.org/version/1.1\",
            \"title:\" \"News\",
            \"homepage:\" \"https://example.com/\",
            \"feed_url:\" \"https://example.com/feed.json\",
            \"items:\" [
                {
                    \"id:\" \"https://example.com/hello_world.html\",
                    \"content_text:\" \"Hello, World!\",
                    \"url:\" \"https://example.com/hello_world.html\",
                    \"date_published:\" \"2026-06-28T08:55:00Z\",
                    \"authors:\" [
                        {
                            \"name:\" \"Jerry\"
                        }
                    ]
                }
            ],
        }
        ";
        
        assert_eq!(Feed::from_str(feed).unwrap(), expected_feed)
    }
}
