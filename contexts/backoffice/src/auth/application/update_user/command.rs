use shared::domain::bus::command::{Command, CommandError, CommandHandler};
use std::fs::File;

use crate::auth::domain::user::{
    UserFullName, UserId, UserIsAdmin, UserPassword, UserProfilePicture, UserUpdatedAt,
    UserUsername,
};

use super::service::UpdateUser;

pub const UPDATE_USER_COMMAND_TYPE: &str = "auth.update_user.command";

#[derive(Debug)]
pub struct UpdateUserCommand {
    pub id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub profile_picture: Option<Option<String>>,
    pub profile_picture_file: Option<File>,
    pub is_admin: Option<bool>,
    pub updated_at: String,
}

impl Command for UpdateUserCommand {
    fn command_type(&self) -> &'static str {
        UPDATE_USER_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct UpdateUserCommandHandler {
    service: UpdateUser,
}

impl UpdateUserCommandHandler {
    pub fn new(service: UpdateUser) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for UpdateUserCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<UpdateUserCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        let user_id =
            UserId::new(command.id.to_owned()).map_err(|e| CommandError::new(e.to_string()))?;
        let user_username = match command.username.clone() {
            Some(username) => {
                Some(UserUsername::new(username).map_err(|e| CommandError::new(e.to_string()))?)
            }
            None => None,
        };
        let user_updated_at = UserUpdatedAt::new(command.updated_at.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;

        let user_password = match command.password.clone() {
            Some(password) => {
                Some(UserPassword::new(password).map_err(|e| CommandError::new(e.to_string()))?)
            }
            None => None,
        };

        let user_full_name = match command.full_name.clone() {
            Some(full_name) => {
                Some(UserFullName::new(full_name).map_err(|e| CommandError::new(e.to_string()))?)
            }
            None => None,
        };
        let user_profile_picture = match command.profile_picture.clone() {
            Some(profile_picture) => Some(
                UserProfilePicture::new(profile_picture)
                    .map_err(|e| CommandError::new(e.to_string()))?,
            ),
            None => None,
        };

        let user_is_admin = command.is_admin.map(UserIsAdmin::new);

        self.service
            .execute(
                user_id,
                user_username,
                user_password,
                user_full_name,
                user_profile_picture,
                command
                    .profile_picture_file
                    .as_ref()
                    .map(|f| f.try_clone())
                    .transpose()
                    .map_err(|e| CommandError::new(e.to_string()))?,
                user_is_admin,
                user_updated_at,
            )
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;

        Ok(())
    }
}
