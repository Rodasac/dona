use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const USER_PAYMENT_METHOD_CREATED_EVENT: &str = "dona.user_payment_method_created";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserPaymentMethodCreatedEvent {
    id: String,
    user_id: String,
    payment_method: String,
    instructions: String,
    created_at: String,
    updated_at: String,

    base_event: BaseEvent,
}

impl UserPaymentMethodCreatedEvent {
    pub fn new(
        id: String,
        user_id: String,
        payment_method: String,
        instructions: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id: id.clone(),
            user_id,
            payment_method,
            instructions,
            created_at,
            updated_at,
            base_event: BaseEvent::new(id),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn payment_method(&self) -> &str {
        &self.payment_method
    }

    pub fn instructions(&self) -> &str {
        &self.instructions
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for UserPaymentMethodCreatedEvent {
    fn event_type(&self) -> &'static str {
        USER_PAYMENT_METHOD_CREATED_EVENT
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn from_primitives(
        &self,
        primitives: EventSerialized,
    ) -> Result<Box<dyn Event>, EventDeserializeError> {
        let data = primitives.data();
        let base_event = BaseEvent::from_primitives(
            primitives.event_id().to_string(),
            primitives.aggregate_id().to_string(),
            primitives.occurred_at().to_string(),
        );

        let id = data
            .get("id")
            .ok_or(EventDeserializeError::MissingField("id".to_string()))?;
        let user_id = data
            .get("user_id")
            .ok_or(EventDeserializeError::MissingField("user_id".to_string()))?;
        let payment_method =
            data.get("payment_method")
                .ok_or(EventDeserializeError::MissingField(
                    "payment_method".to_string(),
                ))?;
        let instructions = data
            .get("instructions")
            .ok_or(EventDeserializeError::MissingField(
                "instructions".to_string(),
            ))?;
        let created_at = data
            .get("created_at")
            .ok_or(EventDeserializeError::MissingField(
                "created_at".to_string(),
            ))?;
        let updated_at = data
            .get("updated_at")
            .ok_or(EventDeserializeError::MissingField(
                "updated_at".to_string(),
            ))?;

        Ok(Box::new(UserPaymentMethodCreatedEvent {
            id: id.to_string(),
            user_id: user_id.to_string(),
            payment_method: payment_method.to_string(),
            instructions: instructions.to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            base_event,
        }))
    }

    fn to_primitives(&self) -> EventSerialized {
        EventSerialized::new(
            self.base_event.event_id().to_string(),
            self.base_event.aggregate_id().to_string(),
            self.base_event.occurred_at().to_string(),
            vec![
                ("id".to_string(), self.id.to_string()),
                ("user_id".to_string(), self.user_id.to_string()),
                (
                    "payment_method".to_string(),
                    self.payment_method.to_string(),
                ),
                ("instructions".to_string(), self.instructions.to_string()),
                ("created_at".to_string(), self.created_at.to_string()),
                ("updated_at".to_string(), self.updated_at.to_string()),
            ]
            .into_iter()
            .collect(),
        )
    }
}
