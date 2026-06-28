mod attachment;
mod errors;

use crate::errors::AttachmentBuildError as AttachmentBuildError;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn attachment_build_error_displays_correctly() {
        assert_eq!(format!("{}", AttachmentBuildError::URLNotFound), "URL must be specified");
    }
}
