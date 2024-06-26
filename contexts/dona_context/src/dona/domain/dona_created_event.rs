use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const DONA_CREATED_EVENT_TYPE: &str = "dona.dona_created";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaCreatedEvent {
    id: String,
    msg: String,
    amount: String,
    user_id: String,
    sender_id: String,
    created_at: String,
    updated_at: String,

    base_event: BaseEvent,
}

impl DonaCreatedEvent {
    pub fn new(
        id: String,
        msg: String,
        amount: String,
        user_id: String,
        sender_id: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id: id.clone(),
            msg,
            amount,
            user_id,
            sender_id,
            created_at,
            updated_at,
            base_event: BaseEvent::new(id),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn amount(&self) -> &str {
        &self.amount
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn sender_id(&self) -> &str {
        &self.sender_id
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
        let id = data
            .get("id")
            .ok_or(EventDeserializeError::MissingField("id".to_string()))?;
        let msg = data
            .get("msg")
            .ok_or(EventDeserializeError::MissingField("msg".to_string()))?;
        let amount = data
            .get("amount")
            .ok_or(EventDeserializeError::MissingField("amount".to_string()))?;
        let user_id = data
            .get("user_id")
            .ok_or(EventDeserializeError::MissingField("user_id".to_string()))?;
        let sender_id = data
            .get("sender_id")
            .ok_or(EventDeserializeError::MissingField("sender_id".to_string()))?;
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
            id: id.to_string(),
            msg: msg.to_string(),
            amount: amount.to_string(),
            user_id: user_id.to_string(),
            sender_id: sender_id.to_string(),
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
                ("id".to_string(), self.id.clone()),
                ("msg".to_string(), self.msg.clone()),
                ("amount".to_string(), self.amount.clone()),
                ("user_id".to_string(), self.user_id.clone()),
                ("sender_id".to_string(), self.sender_id.clone()),
                ("created_at".to_string(), self.created_at.clone()),
                ("updated_at".to_string(), self.updated_at.clone()),
            ]
            .into_iter()
            .collect(),
        )
    }
}
