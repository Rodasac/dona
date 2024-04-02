use std::sync::Arc;

use crate::user_payment_method::{
    application::response::UserPaymentMethodResponse,
    domain::{
        user_payment_method::UserPaymentMethodId,
        user_payment_method_repository::UserPaymentMethodRepository,
    },
};

#[derive(Clone)]
pub struct UserPaymentMethodFinder {
    user_payment_method_repository: Arc<dyn UserPaymentMethodRepository>,
}

impl UserPaymentMethodFinder {
    pub fn new(user_payment_method_repository: Arc<dyn UserPaymentMethodRepository>) -> Self {
        Self {
            user_payment_method_repository,
        }
    }

    pub async fn execute(&self, id: String) -> Result<UserPaymentMethodResponse, String> {
        let user_payment_method = self
            .user_payment_method_repository
            .find_by_id(UserPaymentMethodId::new(id)?)
            .await
            .map_err(|e| e.to_string())?;

        Ok(UserPaymentMethodResponse {
            id: user_payment_method.id().to_string(),
            user_id: user_payment_method.user_id().to_string(),
            payment_method: user_payment_method.payment_method().to_string(),
            instructions: user_payment_method.instructions().to_string(),
            created_at: user_payment_method.created_at(),
            updated_at: user_payment_method.updated_at(),
        })
    }
}
