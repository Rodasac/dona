use shared::domain::bus::query::{Query, QueryError, QueryHandler, Response};

use super::service::UserPaymentMethodFinder;

pub const FIND_USER_PAYMENT_METHOD_QUERY_TYPE: &str = "dona.find_user_payment_methods.query";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FindUserPaymentMethodQuery {
    pub user_id: String,
}

impl Query for FindUserPaymentMethodQuery {
    fn query_type(&self) -> &'static str {
        FIND_USER_PAYMENT_METHOD_QUERY_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct FindUserPaymentMethodQueryHandler {
    service: UserPaymentMethodFinder,
}

impl FindUserPaymentMethodQueryHandler {
    pub fn new(service: UserPaymentMethodFinder) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl QueryHandler for FindUserPaymentMethodQueryHandler {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query = query
            .as_any()
            .downcast_ref::<FindUserPaymentMethodQuery>()
            .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

        let user_payment_method = self
            .service
            .execute(query.user_id.to_owned())
            .await
            .map_err(|e| QueryError::new(e.to_string()))?;

        Ok(Box::new(user_payment_method))
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use std::sync::Arc;

    use super::*;

    use crate::user_payment_method::application::response::UserPaymentMethodResponse;
    use crate::user_payment_method::domain::user_payment_method::tests::{
        UserPaymentMethodIdMother, UserPaymentMethodMother,
    };
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;

    #[tokio::test]
    async fn it_should_return_error_when_user_payment_methods_not_found() {
        let user_id = UserPaymentMethodIdMother::random();

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .return_const(Err(BaseRepositoryError::NotFound));

        let user_payment_method_finder =
            UserPaymentMethodFinder::new(Arc::new(user_payment_method_repository));
        let query_handler = FindUserPaymentMethodQueryHandler::new(user_payment_method_finder);

        let query = FindUserPaymentMethodQuery {
            user_id: user_id.to_string(),
        };
        let result = query_handler.handle(Box::new(query)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn it_should_return_user_payment_method() {
        let user = UserPaymentMethodMother::random();

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_id()
            .with(predicate::eq(UserPaymentMethodIdMother::create(Some(
                user.id(),
            ))))
            .return_const(Ok(user.clone()));

        let user_payment_method_finder =
            UserPaymentMethodFinder::new(Arc::new(user_payment_method_repository));
        let query_handler = FindUserPaymentMethodQueryHandler::new(user_payment_method_finder);

        let query = FindUserPaymentMethodQuery {
            user_id: user.id().to_string(),
        };
        let response = query_handler.handle(Box::new(query)).await.unwrap();
        let response = response
            .as_any()
            .downcast_ref::<UserPaymentMethodResponse>()
            .unwrap();

        assert_eq!(response.id, user.id());
    }
}
