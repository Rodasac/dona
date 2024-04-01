use shared::domain::bus::command::{Command, CommandError, CommandHandler};
use time::OffsetDateTime;

use super::service::UserPaymentMethodInstructionsUpdater;

pub const UPDATE_USER_PAYMENT_METHOD_INSTRUCTIONS_COMMAND_TYPE: &str =
    "dona.update_user_payment_method_instructions.command";

#[derive(Debug)]
pub struct UpdateUserPaymentMethodInstructionsCommand {
    pub id: String,
    pub user_id: String,
    pub instructions: String,
    pub updated_at: OffsetDateTime,
}

impl Command for UpdateUserPaymentMethodInstructionsCommand {
    fn command_type(&self) -> &'static str {
        UPDATE_USER_PAYMENT_METHOD_INSTRUCTIONS_COMMAND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct UpdateUserPaymentMethodInstructionsCommandHandler {
    service: UserPaymentMethodInstructionsUpdater,
}

impl UpdateUserPaymentMethodInstructionsCommandHandler {
    pub fn new(service: UserPaymentMethodInstructionsUpdater) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl CommandHandler for UpdateUserPaymentMethodInstructionsCommandHandler {
    async fn handle(&self, command: Box<dyn Command>) -> Result<(), CommandError> {
        let command = command
            .as_any()
            .downcast_ref::<UpdateUserPaymentMethodInstructionsCommand>()
            .ok_or_else(|| CommandError::new("Invalid command".to_string()))?;

        self.service
            .execute(
                command.id.to_owned(),
                command.user_id.to_owned(),
                command.instructions.to_owned(),
                command.updated_at,
            )
            .await
            .map_err(|e| CommandError::new(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::user_payment_method::domain::user_payment_method::tests::UserPaymentMethodMother;
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;
    use std::sync::Arc;

    use super::*;

    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use shared::domain::bus::event::tests::MockEventBus;
    use shared::domain::criteria::filter::{Filter, FilterField, FilterOperator, FilterValue};
    use shared::domain::criteria::Criteria;

    #[tokio::test]
    async fn it_should_fail_when_user_payment_method_not_found() {
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_criteria()
            .times(1)
            .with(predicate::eq(Criteria::new(
                vec![
                    Filter::new(
                        FilterField::try_from("id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from("id".to_string()).unwrap(),
                    ),
                    Filter::new(
                        FilterField::try_from("user_id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from("user_id".to_string()).unwrap(),
                    ),
                ],
                None,
                None,
            )))
            .returning(|_| Ok(vec![]));
        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(0);

        let service =
            UserPaymentMethodInstructionsUpdater::new(Arc::new(repository), Arc::new(event_bus));
        let handler = UpdateUserPaymentMethodInstructionsCommandHandler::new(service);

        let command = UpdateUserPaymentMethodInstructionsCommand {
            id: "id".to_string(),
            user_id: "user_id".to_string(),
            instructions: "instructions".to_string(),
            updated_at: OffsetDateTime::now_utc(),
        };

        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "CommandError: User payment method not found"
        );
    }

    #[tokio::test]
    async fn it_should_fail_when_repository_fails() {
        let method = UserPaymentMethodMother::random();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_criteria()
            .times(1)
            .with(predicate::eq(Criteria::new(
                vec![
                    Filter::new(
                        FilterField::try_from("id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(method.id().to_string()).unwrap(),
                    ),
                    Filter::new(
                        FilterField::try_from("user_id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(method.user_id().to_string()).unwrap(),
                    ),
                ],
                None,
                None,
            )))
            .return_const(Ok(vec![method.clone()]));

        repository
            .expect_save()
            .times(1)
            .return_const(Err(BaseRepositoryError::UnexpectedError(
                "error".to_string(),
            )));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(0);

        let service =
            UserPaymentMethodInstructionsUpdater::new(Arc::new(repository), Arc::new(event_bus));
        let handler = UpdateUserPaymentMethodInstructionsCommandHandler::new(service);

        let command = UpdateUserPaymentMethodInstructionsCommand {
            id: method.id().to_string(),
            user_id: method.user_id().to_string(),
            instructions: "instructions".to_string(),
            updated_at: OffsetDateTime::now_utc(),
        };

        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "CommandError: Unexpected error: error"
        );
    }

    #[tokio::test]
    async fn it_should_update_user_payment_method_instructions() {
        let method = UserPaymentMethodMother::random();
        let updated_at = OffsetDateTime::now_utc();
        let mut repository = MockUserPaymentMethodRepository::new();
        repository
            .expect_find_by_criteria()
            .times(1)
            .with(predicate::eq(Criteria::new(
                vec![
                    Filter::new(
                        FilterField::try_from("id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(method.id().to_string()).unwrap(),
                    ),
                    Filter::new(
                        FilterField::try_from("user_id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(method.user_id().to_string()).unwrap(),
                    ),
                ],
                None,
                None,
            )))
            .return_const(Ok(vec![method.clone()]));

        let mut method_clone = method.clone();
        method_clone
            .update_instructions("instructions".to_string(), updated_at)
            .unwrap();
        repository
            .expect_save()
            .times(1)
            .with(predicate::eq(method_clone))
            .return_const(Ok(()));

        let mut event_bus = MockEventBus::new();
        event_bus.expect_publish().times(1).return_const(Ok(()));

        let service =
            UserPaymentMethodInstructionsUpdater::new(Arc::new(repository), Arc::new(event_bus));
        let handler = UpdateUserPaymentMethodInstructionsCommandHandler::new(service);

        let command = UpdateUserPaymentMethodInstructionsCommand {
            id: method.id().to_string(),
            user_id: method.user_id().to_string(),
            instructions: "instructions".to_string(),
            updated_at: updated_at,
        };

        let result = handler.handle(Box::new(command)).await;

        assert!(result.is_ok());
    }
}
