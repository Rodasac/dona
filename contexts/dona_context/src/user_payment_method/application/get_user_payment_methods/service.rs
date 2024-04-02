use std::sync::Arc;

use shared::domain::criteria::{
    filter::{Filter, FilterField, FilterOperator, FilterValue},
    Criteria,
};

use crate::user_payment_method::{
    application::response::{UserPaymentMethodResponse, UserPaymentMethodsResponse},
    domain::user_payment_method_repository::UserPaymentMethodRepository,
};

#[derive(Clone)]
pub struct GetPaymentMethodsByUser {
    user_payment_method_repository: Arc<dyn UserPaymentMethodRepository>,
}

impl GetPaymentMethodsByUser {
    pub fn new(user_payment_method_repository: Arc<dyn UserPaymentMethodRepository>) -> Self {
        Self {
            user_payment_method_repository,
        }
    }

    fn criteria(&self, user_id: String) -> Result<Criteria, String> {
        Ok(Criteria::new(
            vec![Filter::new(
                FilterField::try_from("user_id".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(user_id)?,
            )],
            None,
            None,
        ))
    }

    pub async fn execute(&self, user_id: String) -> Result<UserPaymentMethodsResponse, String> {
        let criteria = self.criteria(user_id)?;
        let user_payment_methods = self
            .user_payment_method_repository
            .find_by_criteria(criteria)
            .await?;

        Ok(UserPaymentMethodsResponse {
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
        })
    }
}
