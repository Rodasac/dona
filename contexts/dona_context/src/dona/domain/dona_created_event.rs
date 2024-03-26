use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const DONA_CREATED_EVENT_TYPE: &str = "dona.dona_created";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaCreatedEvent {
    dona_id: String,
    dona_msg: String,
    dona_amount: String,
    dona_user_id: String,
    dona_sender_id: String,
    created_at: String,
    updated_at: String,

    base_event: BaseEvent,
}

impl DonaCreatedEvent {
    pub fn new(
        dona_id: String,
        dona_msg: String,
        dona_amount: String,
        dona_user_id: String,
        dona_sender_id: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            dona_id: dona_id.clone(),
            dona_msg,
            dona_amount,
            dona_user_id,
            dona_sender_id,
            created_at,
            updated_at,
            base_event: BaseEvent::new(dona_id),
        }
    }

    pub fn dona_id(&self) -> &str {
        &self.dona_id
    }

    pub fn dona_msg(&self) -> &str {
        &self.dona_msg
    }

    pub fn dona_amount(&self) -> &str {
        &self.dona_amount
    }

    pub fn dona_user_id(&self) -> &str {
        &self.dona_user_id
    }

    pub fn dona_sender_id(&self) -> &str {
        &self.dona_sender_id
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for DonaCreatedEvent {
    fn event_type(&self) -> &'static str {
        DONA_CREATED_EVENT_TYPE
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
        let dona_id = data
            .get("dona_id")
            .ok_or(EventDeserializeError::MissingField("dona_id".to_string()))?;
        let dona_msg = data
            .get("dona_msg")
            .ok_or(EventDeserializeError::MissingField("dona_msg".to_string()))?;
        let dona_amount = data
            .get("dona_amount")
            .ok_or(EventDeserializeError::MissingField(
                "dona_amount".to_string(),
            ))?;
        let dona_user_id = data
            .get("dona_user_id")
            .ok_or(EventDeserializeError::MissingField(
                "dona_user_id".to_string(),
            ))?;
        let dona_sender_id =
            data.get("dona_sender_id")
                .ok_or(EventDeserializeError::MissingField(
                    "dona_sender_id".to_string(),
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

        Ok(Box::new(Self {
            dona_id: dona_id.to_string(),
            dona_msg: dona_msg.to_string(),
            dona_amount: dona_amount.to_string(),
            dona_user_id: dona_user_id.to_string(),
            dona_sender_id: dona_sender_id.to_string(),
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
                ("dona_id".to_string(), self.dona_id.clone()),
                ("dona_msg".to_string(), self.dona_msg.clone()),
                ("dona_amount".to_string(), self.dona_amount.clone()),
                ("dona_user_id".to_string(), self.dona_user_id.clone()),
                ("dona_sender_id".to_string(), self.dona_sender_id.clone()),
                ("created_at".to_string(), self.created_at.clone()),
                ("updated_at".to_string(), self.updated_at.clone()),
            ]
            .into_iter()
            .collect(),
        )
    }
}
