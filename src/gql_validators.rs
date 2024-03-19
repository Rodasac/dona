use async_graphql::{Error, UploadValue};
use poem::session::Session;
use security::session::application::check_permission::command::CheckPermissionCommand;

use crate::{CommandBusType, MAX_UPLOAD_SIZE};

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

pub fn is_authenticated(session: &Session) -> bool {
    session.get::<String>("session_id").is_some()
}

pub fn is_authenticated_with_err(session: &Session) -> Result<(), Error> {
    if is_authenticated(session) {
        Ok(())
    } else {
        Err(Error::new("UNAUTHORIZED"))
    }
}

pub async fn check_admin(bus: &CommandBusType, session: &Session) -> Result<(), Error> {
    let session_id = session
        .get::<String>("session_id")
        .ok_or(Error::new("UNAUTHORIZED"))?;
    let check_admin_command = CheckPermissionCommand {
        session_id: session_id.clone(),
        user_id: None,
    };

    bus.dispatch(Box::new(check_admin_command))
        .await
        .map_err(|e| Error::new(e.to_string()))
}

pub async fn check_permission(bus: &CommandBusType, session: &Session) -> Result<(), Error> {
    let session_id = session
        .get::<String>("session_id")
        .ok_or(Error::new("UNAUTHORIZED"))?;
    let user_id = session.get::<String>("user_id");
    let check_perm_command = CheckPermissionCommand {
        session_id,
        user_id,
    };

    bus.dispatch(Box::new(check_perm_command))
        .await
        .map_err(|e| Error::new(e.to_string()))
}
