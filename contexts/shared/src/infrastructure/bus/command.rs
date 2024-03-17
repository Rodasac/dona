use std::{collections::HashMap, sync::Arc};

use crate::domain::bus::command::{Command, CommandBus, CommandError, CommandHandler};

#[derive(Clone, Default)]
pub struct InMemoryCommandBus {
    handlers: HashMap<&'static str, Arc<dyn CommandHandler>>,
}

impl InMemoryCommandBus {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl CommandBus for InMemoryCommandBus {
    async fn dispatch(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command_type = command.command_type();
        let handler = self.handlers.get(command_type).ok_or_else(|| {
            CommandError::new(format!("No handler found for command: {}", command_type))
        })?;
        handler.handle(command).await
    }

    fn register_handler(&mut self, command_type: &'static str, handler: Arc<dyn CommandHandler>) {
        self.handlers.insert(command_type, handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCommand;

    impl Command for TestCommand {
        fn command_type(&self) -> &'static str {
            "TestCommand"
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCommandHandler;

    #[async_trait::async_trait]
    impl CommandHandler for TestCommandHandler {
        async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
            let command = command
                .as_any()
                .downcast_ref::<TestCommand>()
                .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

            assert_eq!(command, &TestCommand);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_in_memory_command_bus() {
        let mut bus = InMemoryCommandBus::new();
        let handler = Arc::new(TestCommandHandler);
        bus.register_handler("TestCommand", handler);

        let command = Box::new(TestCommand);
        bus.dispatch(command).await.unwrap();
    }
}
