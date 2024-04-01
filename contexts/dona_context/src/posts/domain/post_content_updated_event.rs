use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const POST_CONTENT_UPDATED_EVENT_TYPE: &str = "dona.post_content_updated";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostContentUpdatedEvent {
    id: String,
    user_id: String,
    content: String,
    updated_at: String,

    base_event: BaseEvent,
}

impl PostContentUpdatedEvent {
    pub fn new(id: String, user_id: String, content: String, updated_at: String) -> Self {
        Self {
            id: id.clone(),
            user_id,
            content,
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

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for PostContentUpdatedEvent {
    fn event_type(&self) -> &'static str {
        POST_CONTENT_UPDATED_EVENT_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_primitives(&self) -> EventSerialized {
        EventSerialized::new(
            self.base_event.event_id().to_string(),
            self.base_event.aggregate_id().to_string(),
            self.base_event.occurred_at().to_string(),
            vec![
                ("id".to_string(), self.id.clone()),
                ("user_id".to_string(), self.user_id.clone()),
                ("content".to_string(), self.content.clone()),
                ("updated_at".to_string(), self.updated_at.clone()),
            ]
            .into_iter()
            .collect(),
        )
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
        let content = data
            .get("content")
            .ok_or(EventDeserializeError::MissingField("content".to_string()))?;
        let updated_at = data
            .get("updated_at")
            .ok_or(EventDeserializeError::MissingField(
                "updated_at".to_string(),
            ))?;

        Ok(Box::new(Self {
            id: id.to_string(),
            user_id: user_id.to_string(),
            content: content.to_string(),
            updated_at: updated_at.to_string(),
            base_event,
        }))
    }
}
