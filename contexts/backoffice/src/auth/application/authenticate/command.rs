use serde::{Deserialize, Serialize};
use shared::domain::bus::command::{Command, CommandError, CommandHandler};

use crate::auth::domain::user::{UserEmail, UserPassword};

use super::service::UserAuthenticator;

pub const AUTHENTICATE_USER_COMMAND_TYPE: &str = "auth.authenticate_user.command";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthenticateUserCommand {
    pub email: String,
    pub password: String,
}

impl Command for AuthenticateUserCommand {
    fn command_type(&self) -> &'static str {
        AUTHENTICATE_USER_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct AuthenticateUserCommandHandler {
    service: UserAuthenticator,
}

impl AuthenticateUserCommandHandler {
    pub fn new(service: UserAuthenticator) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for AuthenticateUserCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<AuthenticateUserCommand>()
            .ok_or_else(|| CommandError::new("Invalid query".to_string()))?;

        let user_email = UserEmail::new(command.email.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;
        let user_password = UserPassword::new(command.password.to_owned())
            .map_err(|e| CommandError::new(e.to_string()))?;

        self.service
            .execute(user_email, user_password)
            .await
            .map_err(|e| CommandError::new(e.to_string()))
    }
}
