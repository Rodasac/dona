use shared::domain::bus::query::{Query, QueryError, QueryHandler, Response};

use super::service::GetPaymentMethodsByUser;

pub const GET_USER_PAYMENT_METHODS_QUERY_TYPE: &str = "dona.get_user_payment_methods.query";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetUserPaymentMethodsQuery {
    pub user_id: String,
}

impl Query for GetUserPaymentMethodsQuery {
    fn query_type(&self) -> &'static str {
        GET_USER_PAYMENT_METHODS_QUERY_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct GetUserPaymentMethodsQueryHandler {
    service: GetPaymentMethodsByUser,
}

impl GetUserPaymentMethodsQueryHandler {
    pub fn new(service: GetPaymentMethodsByUser) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl QueryHandler for GetUserPaymentMethodsQueryHandler {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query = query
            .as_any()
            .downcast_ref::<GetUserPaymentMethodsQuery>()
            .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

        let user_payment_methods = self
            .service
            .execute(query.user_id.to_owned())
            .await
            .map_err(|e| QueryError::new(e.to_string()))?;

        Ok(Box::new(user_payment_methods))
    }
}

#[cfg(test)]
mod tests {
    use shared::domain::base_errors::BaseRepositoryError;
    use std::sync::Arc;

    use super::*;

    use crate::user_payment_method::application::response::{
        UserPaymentMethodResponse, UserPaymentMethodsResponse,
    };
    use crate::user_payment_method::domain::user_payment_method::tests::UserPaymentMethodMother;
    use crate::user_payment_method::domain::user_payment_method_repository::tests::MockUserPaymentMethodRepository;

    #[tokio::test]
    async fn it_should_return_error_when_repository_fails() {
        let user_id = "user_id".to_string();

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(move |_| Err(BaseRepositoryError::UnexpectedError("Error".to_string())));

        let service = GetPaymentMethodsByUser::new(Arc::new(user_payment_method_repository));
        let handler = GetUserPaymentMethodsQueryHandler::new(service);

        let query = GetUserPaymentMethodsQuery { user_id };
        let response = handler.handle(Box::new(query)).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn it_should_return_user_payment_methods() {
        let user_id = "user_id".to_string();
        let user_payment_method = UserPaymentMethodMother::random();
        let user_payment_methods = vec![user_payment_method.clone()];

        let mut user_payment_method_repository = MockUserPaymentMethodRepository::new();
        user_payment_method_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(move |_| Ok(user_payment_methods.clone()));

        let service = GetPaymentMethodsByUser::new(Arc::new(user_payment_method_repository));
        let handler = GetUserPaymentMethodsQueryHandler::new(service);

        let query = GetUserPaymentMethodsQuery { user_id };
        let response = handler.handle(Box::new(query)).await.unwrap();
        let response = response
            .as_any()
            .downcast_ref::<UserPaymentMethodsResponse>()
            .unwrap();

        let expected_response = UserPaymentMethodsResponse {
            users: vec![UserPaymentMethodResponse {
                id: user_payment_method.id().to_string(),
                user_id: user_payment_method.user_id().to_string(),
                payment_method: user_payment_method.payment_method().to_string(),
                instructions: user_payment_method.instructions().to_string(),
                created_at: user_payment_method.created_at(),
                updated_at: user_payment_method.updated_at(),
            }],
        };

        assert_eq!(response.to_owned(), expected_response);
    }
}
