use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const DONA_CONFIRMED_EVENT_TYPE: &str = "dona.dona_confirmed";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaConfirmedEvent {
    dona_id: String,
    dona_updated_at: String,

    base_event: BaseEvent,
}

impl DonaConfirmedEvent {
    pub fn new(dona_id: String, dona_updated_at: String) -> Self {
        Self {
            dona_id: dona_id.clone(),
            dona_updated_at,
            base_event: BaseEvent::new(dona_id),
        }
    }

    pub fn dona_id(&self) -> &str {
        &self.dona_id
    }

    pub fn dona_updated_at(&self) -> &str {
        &self.dona_updated_at
    }
}

impl Event for DonaConfirmedEvent {
    fn event_type(&self) -> &'static str {
        DONA_CONFIRMED_EVENT_TYPE
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
        let updated_at = data
            .get("dona_updated_at")
            .ok_or(EventDeserializeError::MissingField(
                "dona_updated_at".to_string(),
            ))?;

        Ok(Box::new(Self {
            dona_id: dona_id.to_string(),
            dona_updated_at: updated_at.to_string(),
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
                ("dona_updated_at".to_string(), self.dona_updated_at.clone()),
            ]
            .into_iter()
            .collect(),
        )
    }
}
