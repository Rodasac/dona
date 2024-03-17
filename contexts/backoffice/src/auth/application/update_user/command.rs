use serde::{Deserialize, Serialize};
use shared::common::domain::bus::command::{Command, CommandError, CommandHandler};

use crate::auth::domain::user::{UserFullName, UserId, UserIsAdmin, UserPassword, UserUpdatedAt};

use super::service::UpdateUser;

pub const UPDATE_USER_COMMAND_TYPE: &str = "auth.update_user.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub id: String,
    pub password: Option<String>,
    pub full_name: Option<String>,
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

        let user_is_admin = command.is_admin.map(UserIsAdmin::new);

        self.service
            .execute(
                user_id,
                user_password,
                user_full_name,
                user_is_admin,
                user_updated_at,
            )
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;

        Ok(())
    }
}
