use serde::{Deserialize, Serialize};
use shared::domain::bus::command::{Command, CommandError, CommandHandler};

use super::service::IsAuthenticatedChecker;

pub const CHECK_IS_AUTHENTICATED_COMMAND_TYPE: &str = "security.check_is_authenticated.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckIsAuthenticatedCommand {
    pub session_id: String,
}

impl Command for CheckIsAuthenticatedCommand {
    fn command_type(&self) -> &'static str {
        CHECK_IS_AUTHENTICATED_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct CheckIsAuthenticatedCommandHandler {
    service: IsAuthenticatedChecker,
}

impl CheckIsAuthenticatedCommandHandler {
    pub fn new(service: IsAuthenticatedChecker) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CheckIsAuthenticatedCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<CheckIsAuthenticatedCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(command.session_id.to_owned())
            .await
            .map_err(|e| CommandError::new(e))
    }
}
