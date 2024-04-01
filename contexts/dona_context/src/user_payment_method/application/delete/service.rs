use std::sync::Arc;

use shared::domain::bus::event::EventBus;

use crate::user_payment_method::domain::{
    user_payment_method::UserPaymentMethodId,
    user_payment_method_repository::UserPaymentMethodRepository,
};

#[derive(Clone)]
pub struct UserPaymentMethodDeleter {
    repository: Arc<dyn UserPaymentMethodRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserPaymentMethodDeleter {
    pub fn new(
        repository: Arc<dyn UserPaymentMethodRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: String) -> Result<(), String> {
        self.repository
            .delete(UserPaymentMethodId::new(id)?)
            .await?;

        self.event_bus.publish(vec![]).await?;

        Ok(())
    }
}
