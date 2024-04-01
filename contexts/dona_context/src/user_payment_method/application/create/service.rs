use std::sync::Arc;

use shared::domain::{base_errors::BaseRepositoryError, bus::event::EventBus};
use time::OffsetDateTime;

use crate::user_payment_method::domain::{
    user_payment_method::{UserPaymentMethod, UserPaymentMethodId},
    user_payment_method_repository::UserPaymentMethodRepository,
};

#[derive(Clone)]
pub struct UserPaymentMethodCreator {
    repository: Arc<dyn UserPaymentMethodRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserPaymentMethodCreator {
    pub fn new(
        repository: Arc<dyn UserPaymentMethodRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    async fn method_exists(&self, id: String) -> Result<(), String> {
        let method = self
            .repository
            .find_by_id(UserPaymentMethodId::new(id)?)
            .await;

        match method {
            Ok(_) => Err("User payment method already exists".to_string()),
            Err(BaseRepositoryError::NotFound) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn execute(
        &self,
        id: String,
        user_id: String,
        payment_method: String,
        instructions: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        self.method_exists(id.clone()).await?;

        let mut user_payment_method = UserPaymentMethod::create(
            id,
            user_id,
            payment_method,
            instructions,
            created_at,
            updated_at,
        )?;

        self.repository
            .save(&user_payment_method)
            .await
            .map_err(|e| e.to_string())?;

        self.event_bus
            .publish(user_payment_method.pull_events())
            .await?;

        Ok(())
    }
}
