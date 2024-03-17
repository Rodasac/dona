use serde::{Deserialize, Serialize};
use shared::domain::bus::command::{Command, CommandError, CommandHandler};

use super::service::PermissionChecker;

pub const CHECK_PERMISSION_COMMAND_TYPE: &str = "security.check_permission.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckPermissionCommand {
    pub session_id: String,
    pub user_id: Option<String>,
}

impl Command for CheckPermissionCommand {
    fn command_type(&self) -> &'static str {
        CHECK_PERMISSION_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct CheckPermissionCommandHandler {
    service: PermissionChecker,
}

impl CheckPermissionCommandHandler {
    pub fn new(service: PermissionChecker) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CheckPermissionCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<CheckPermissionCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(command.session_id.to_owned(), command.user_id.to_owned())
            .await
            .map_err(|e| CommandError::new(e))
    }
}
