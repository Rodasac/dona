use shared::domain::{
    bus::query::{Query, QueryError, QueryHandler, Response},
    criteria::Criteria,
};

use super::service::UserPaymentMethodsFinder;

pub const FIND_USER_PAYMENT_METHODS_BY_CRITERIA_QUERY_TYPE: &str =
    "dona.find_user_payment_methods.query";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FindUserPaymentMethodsQuery {
    pub criteria: Criteria,
}

impl Query for FindUserPaymentMethodsQuery {
    fn query_type(&self) -> &'static str {
        FIND_USER_PAYMENT_METHODS_BY_CRITERIA_QUERY_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct FindUserPaymentMethodsQueryHandler {
    service: UserPaymentMethodsFinder,
}

impl FindUserPaymentMethodsQueryHandler {
    pub fn new(service: UserPaymentMethodsFinder) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl QueryHandler for FindUserPaymentMethodsQueryHandler {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query = query
            .as_any()
            .downcast_ref::<FindUserPaymentMethodsQuery>()
            .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

        let user_payment_methods = self
            .service
            .execute(query.criteria.to_owned())
            .await
            .map_err(|e| QueryError::new(e.to_string()))?;

        Ok(Box::new(user_payment_methods))
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use std::sync::Arc;

    use super::*;

    use crate::user_payment_method::application::response::{
        UserPaymentMethodResponse, UserPaymentMethodsResponse,
    };
    use crate::user_payment_method::domain::user_payment_method::tests::UserPaymentMethodMother;
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;

    fn mock_criteria() -> Criteria {
        Criteria::new(vec![], None, None)
    }

    #[tokio::test]
    async fn it_should_return_error_when_user_payment_methods_repo_fails() {
        let criteria = mock_criteria();

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_criteria()
            .with(predicate::eq(criteria.clone()))
            .return_const(Err(BaseRepositoryError::UnexpectedError(
                "Error".to_string(),
            )));

        let user_payment_methods_finder =
            UserPaymentMethodsFinder::new(Arc::new(user_payment_method_repository));

        let query = FindUserPaymentMethodsQuery { criteria };

        let handler = FindUserPaymentMethodsQueryHandler::new(user_payment_methods_finder);

        let response = handler.handle(Box::new(query)).await;

        assert!(response.is_err(), "Expected error response");
    }

    #[tokio::test]
    async fn it_should_return_user_payment_methods() {
        let criteria = mock_criteria();
        let user_payment_methods = vec![UserPaymentMethodMother::random()];

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_criteria()
            .with(predicate::eq(criteria.clone()))
            .return_const(Ok(user_payment_methods.clone()));

        let user_payment_methods_finder =
            UserPaymentMethodsFinder::new(Arc::new(user_payment_method_repository));

        let query = FindUserPaymentMethodsQuery { criteria };

        let handler = FindUserPaymentMethodsQueryHandler::new(user_payment_methods_finder);

        let response = handler.handle(Box::new(query)).await.unwrap();
        let response = response
            .as_any()
            .downcast_ref::<UserPaymentMethodsResponse>()
            .unwrap()
            .to_owned();

        let expected_response = UserPaymentMethodsResponse {
            users: user_payment_methods
                .into_iter()
                .map(|user_payment_method| UserPaymentMethodResponse {
                    id: user_payment_method.id().to_string(),
                    user_id: user_payment_method.user_id().to_string(),
                    payment_method: user_payment_method.payment_method().to_string(),
                    instructions: user_payment_method.instructions().to_string(),
                    created_at: user_payment_method.created_at(),
                    updated_at: user_payment_method.updated_at(),
                })
                .collect(),
        };

        assert_eq!(response, expected_response);
    }
}
