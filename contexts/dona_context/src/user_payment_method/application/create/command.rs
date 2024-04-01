use shared::domain::bus::command::{Command, CommandError, CommandHandler};
use time::OffsetDateTime;

use super::service::UserPaymentMethodCreator;

pub const CREATE_USER_PAYMENT_METHOD_COMMAND_TYPE: &str = "dona.create_user_payment_method.command";

#[derive(Debug)]
pub struct CreateUserPaymentMethodCommand {
    pub id: String,
    pub user_id: String,
    pub payment_method: String,
    pub instructions: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Command for CreateUserPaymentMethodCommand {
    fn command_type(&self) -> &'static str {
        CREATE_USER_PAYMENT_METHOD_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct CreateUserPaymentMethodCommandHandler {
    service: UserPaymentMethodCreator,
}

impl CreateUserPaymentMethodCommandHandler {
    pub fn new(service: UserPaymentMethodCreator) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateUserPaymentMethodCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<CreateUserPaymentMethodCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(
                command.id.to_owned(),
                command.user_id.to_owned(),
                command.payment_method.to_owned(),
                command.instructions.to_owned(),
                command.created_at,
                command.updated_at,
            )
            .await
            .map_err(|e| CommandError::new(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::user_payment_method::domain::user_payment_method::tests::UserPaymentMethodMother;
    use crate::user_payment_method::domain::user_payment_method::UserPaymentMethodId;
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;
    use std::sync::Arc;

    use super::*;

    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use shared::domain::bus::event::tests::MockEventBus;

    #[tokio::test]
    async fn it_should_fail_when_method_exists() {
        let method = UserPaymentMethodMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_id()
            .with(predicate::eq(
                UserPaymentMethodId::new(method.id()).unwrap(),
            ))
            .times(1)
            .return_const(Ok(method.clone()));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(0);

        let service = UserPaymentMethodCreator::new(Arc::new(repository), Arc::new(event_bus));
        let handler = CreateUserPaymentMethodCommandHandler::new(service);

        let command = CreateUserPaymentMethodCommand {
            id: method.id(),
            user_id: method.user_id(),
            payment_method: method.payment_method(),
            instructions: method.instructions(),
            created_at: method.created_at(),
            updated_at: method.updated_at(),
        };
        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_err(), "Result should be an error");
    }

    #[tokio::test]
    async fn it_should_fail_when_repository_fails() {
        let method = UserPaymentMethodMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_id()
            .with(predicate::eq(
                UserPaymentMethodId::new(method.id()).unwrap(),
            ))
            .times(1)
            .return_const(Err(BaseRepositoryError::NotFound));

        repository
            .expect_save()
            .with(predicate::eq(method.clone()))
            .times(1)
            .return_const(Err(BaseRepositoryError::UnexpectedError(
                "error".to_string(),
            )));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(0);

        let service = UserPaymentMethodCreator::new(Arc::new(repository), Arc::new(event_bus));
        let handler = CreateUserPaymentMethodCommandHandler::new(service);

        let command = CreateUserPaymentMethodCommand {
            id: method.id(),
            user_id: method.user_id(),
            payment_method: method.payment_method(),
            instructions: method.instructions(),
            created_at: method.created_at(),
            updated_at: method.updated_at(),
        };
        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_err(), "Result should be an error");
    }

    #[tokio::test]
    async fn it_should_create_user_payment_method() {
        let method = UserPaymentMethodMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_id()
            .with(predicate::eq(
                UserPaymentMethodId::new(method.id()).unwrap(),
            ))
            .times(1)
            .return_const(Err(BaseRepositoryError::NotFound));

        repository
            .expect_save()
            .with(predicate::eq(method.clone()))
            .times(1)
            .return_const(Ok(()));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(1).return_const(Ok(()));

        let service = UserPaymentMethodCreator::new(Arc::new(repository), Arc::new(event_bus));
        let handler = CreateUserPaymentMethodCommandHandler::new(service);

        let command = CreateUserPaymentMethodCommand {
            id: method.id(),
            user_id: method.user_id(),
            payment_method: method.payment_method(),
            instructions: method.instructions(),
            created_at: method.created_at(),
            updated_at: method.updated_at(),
        };
        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_ok(), "Result should be Ok");
    }
}
