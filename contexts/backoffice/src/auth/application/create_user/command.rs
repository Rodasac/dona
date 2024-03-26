use shared::domain::{
    bus::command::{Command, CommandError, CommandHandler},
    value_objects::user_id::UserId,
};
use std::fs::File;

use crate::auth::domain::user::{
    UserCreatedAt, UserEmail, UserFullName, UserIsAdmin, UserPassword, UserProfilePicture,
    UserUpdatedAt, UserUsername,
};

use super::service::CreateUser;

pub const CREATE_USER_COMMAND_TYPE: &str = "auth.create_user.command";

#[derive(Debug)]
pub struct CreateUserCommand {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_picture: Option<String>,
    pub profile_picture_file: Option<File>,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Command for CreateUserCommand {
    fn command_type(&self) -> &'static str {
        CREATE_USER_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct CreateUserCommandHandler {
    service: CreateUser,
}

impl CreateUserCommandHandler {
    pub fn new(service: CreateUser) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateUserCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<CreateUserCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        let user_id =
            UserId::new(command.id.to_owned()).map_err(|e| CommandError::new(e.to_string()))?;
        let user_username = UserUsername::new(command.email.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_email = UserEmail::new(command.email.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_password = UserPassword::new(command.password.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_full_name = UserFullName::new(command.full_name.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_profile_picture = UserProfilePicture::new(command.profile_picture.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_is_admin = UserIsAdmin::new(command.is_admin);
        let user_created_at = UserCreatedAt::new(command.created_at.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_updated_at = UserUpdatedAt::new(command.updated_at.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;

        self.service
            .execute(
                user_id,
                user_username,
                user_email,
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
                user_created_at,
                user_updated_at,
            )
            .await
            .map_err(|e| CommandError::new(e.to_string()))
    }
}
