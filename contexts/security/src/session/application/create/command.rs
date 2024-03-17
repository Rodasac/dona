use serde::{Deserialize, Serialize};
use shared::domain::bus::command::{Command, CommandError, CommandHandler};
use time::OffsetDateTime;

use super::service::SessionCreator;

pub const CREATE_SESSION_COMMAND_TYPE: &str = "security.create_session.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CreateSessionCommand {
    pub user_id: String,
    pub session_id: String,
    pub login_at: OffsetDateTime,
    pub user_is_admin: bool,
}

impl Command for CreateSessionCommand {
    fn command_type(&self) -> &'static str {
        CREATE_SESSION_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct CreateSessionCommandHandler {
    service: SessionCreator,
}

impl CreateSessionCommandHandler {
    pub fn new(service: SessionCreator) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateSessionCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<CreateSessionCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(
                command.user_id.to_owned(),
                command.session_id.to_owned(),
                command.login_at,
                command.user_is_admin,
            )
            .await
            .map_err(|e| CommandError::new(e))
    }
}
