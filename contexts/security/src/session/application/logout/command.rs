use serde::{Deserialize, Serialize};
use shared::domain::bus::command::{Command, CommandError, CommandHandler};

use super::service::SessionLogout;

pub const LOGOUT_SESSION_COMMAND_TYPE: &str = "security.logout_session.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogoutSessionCommand {
    pub session_id: String,
}

impl Command for LogoutSessionCommand {
    fn command_type(&self) -> &'static str {
        LOGOUT_SESSION_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct LogoutSessionCommandHandler {
    service: SessionLogout,
}

impl LogoutSessionCommandHandler {
    pub fn new(service: SessionLogout) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for LogoutSessionCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<LogoutSessionCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(command.session_id.to_owned())
            .await
            .map_err(|e| CommandError::new(e))
    }
}
