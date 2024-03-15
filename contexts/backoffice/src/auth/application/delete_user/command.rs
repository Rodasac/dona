use serde::{Deserialize, Serialize};
use shared::common::domain::bus::command::{Command, CommandError, CommandHandler};

use crate::auth::domain::user::UserId;

use super::service::UserDeleter;

pub const DELETE_USER_COMMAND_TYPE: &str = "auth.delete_user.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeleteUserCommand {
    pub id: String,
}

impl Command for DeleteUserCommand {
    fn command_type(&self) -> &'static str {
        DELETE_USER_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct DeleteUserCommandHandler {
    service: UserDeleter,
}

impl DeleteUserCommandHandler {
    pub fn new(service: UserDeleter) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for DeleteUserCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<DeleteUserCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        let user_id =
            UserId::new(command.id.to_owned()).map_err(|e| CommandError::new(e.to_string()))?;

        self.service
            .execute(user_id)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;

        Ok(())
    }
}
