use std::sync::Arc;

use shared::domain::{
    bus::event::EventBus,
    criteria::{
        filter::{Filter, FilterField, FilterOperator, FilterValue},
        Criteria,
    },
};
use time::OffsetDateTime;

use crate::user_payment_method::domain::{
    user_payment_method::UserPaymentMethod,
    user_payment_method_repository::UserPaymentMethodRepository,
};

#[derive(Clone)]
pub struct UserPaymentMethodInstructionsUpdater {
    repository: Arc<dyn UserPaymentMethodRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserPaymentMethodInstructionsUpdater {
    pub fn new(
        repository: Arc<dyn UserPaymentMethodRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    async fn method_finder(
        &self,
        id: String,
        user_id: String,
    ) -> Result<UserPaymentMethod, String> {
        Ok(self
            .repository
            .find_by_criteria(Criteria::new(
                vec![
                    Filter::new(
                        FilterField::try_from("id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(id.clone()).unwrap(),
                    ),
                    Filter::new(
                        FilterField::try_from("user_id".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(user_id.clone()).unwrap(),
                    ),
                ],
                None,
                None,
            ))
            .await?
            .pop()
            .ok_or_else(|| "User payment method not found".to_string())?)
    }

    pub async fn execute(
        &self,
        id: String,
        user_id: String,
        instructions: String,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        let mut user_payment_method = self.method_finder(id.clone(), user_id.clone()).await?;
        user_payment_method.update_instructions(instructions, updated_at)?;

        self.repository.save(&user_payment_method).await?;

        self.event_bus
            .publish(user_payment_method.pull_events())
            .await?;

        Ok(())
    }
}
