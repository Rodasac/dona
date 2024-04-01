use shared::domain::bus::command::{Command, CommandError, CommandHandler};

use super::service::UserPaymentMethodDeleter;

pub const DELETE_USER_PAYMENT_METHOD_COMMAND_TYPE: &str = "dona.delete_user_payment_method.command";

#[derive(Debug)]
pub struct DeleteUserPaymentMethodCommand {
    pub id: String,
}

impl Command for DeleteUserPaymentMethodCommand {
    fn command_type(&self) -> &'static str {
        DELETE_USER_PAYMENT_METHOD_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct DeleteUserPaymentMethodCommandHandler {
    service: UserPaymentMethodDeleter,
}

impl DeleteUserPaymentMethodCommandHandler {
    pub fn new(service: UserPaymentMethodDeleter) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for DeleteUserPaymentMethodCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<DeleteUserPaymentMethodCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(command.id.to_owned())
            .await
            .map_err(|e| CommandError::new(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::user_payment_method::domain::user_payment_method::tests::UserPaymentMethodIdMother;
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;
    use std::sync::Arc;

    use super::*;

    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use shared::domain::bus::event::tests::MockEventBus;

    #[tokio::test]
    async fn it_should_fail_to_delete_a_user_payment_method() {
        let id = UserPaymentMethodIdMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_delete()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(0);

        let deleter = UserPaymentMethodDeleter::new(Arc::new(repository), Arc::new(event_bus));
        let handler = DeleteUserPaymentMethodCommandHandler::new(deleter);

        let command = DeleteUserPaymentMethodCommand { id: id.to_string() };
        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn it_should_delete_a_user_payment_method() {
        let id = UserPaymentMethodIdMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_delete()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(1).returning(|_| Ok(()));

        let deleter = UserPaymentMethodDeleter::new(Arc::new(repository), Arc::new(event_bus));
        let handler = DeleteUserPaymentMethodCommandHandler::new(deleter);

        let command = DeleteUserPaymentMethodCommand { id: id.to_string() };
        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_ok());
    }
}
