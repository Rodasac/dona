use async_graphql::{Error, UploadValue};

use crate::MAX_UPLOAD_SIZE;

const IMAGE_TYPES: [&str; 8] = [
    "image/jpg",
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "image/svg+xml",
    "image/avif",
    "image/heic",
];

pub fn check_upload(value: &Option<UploadValue>) -> Result<(), Error> {
    match value {
        Some(value) => {
            if value.size().map_err(|e| Error::new(e.to_string()))? > MAX_UPLOAD_SIZE {
                return Err(Error::new("File too large"));
            }

            if IMAGE_TYPES.contains(&value.content_type.clone().unwrap_or_default().as_str()) {
                Ok(())
            } else {
                Err(Error::new("Invalid image type"))
            }
        }
        None => Ok(()),
    }
}
