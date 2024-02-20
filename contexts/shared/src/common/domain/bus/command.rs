use std::{any::Any, error::Error, fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

pub trait Command: Send + Sync {
    fn command_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandError(String);

impl CommandError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl Error for CommandError {}
impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandError: {}", self.0)
    }
}

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError>;
}

#[async_trait::async_trait]
pub trait CommandBus: Send + Sync {
    async fn dispatch(&self, command: Box<dyn Command>) -> Result<(), CommandError>;
    fn register_handler(&mut self, command_type: &'static str, handler: Arc<dyn CommandHandler>);
}
