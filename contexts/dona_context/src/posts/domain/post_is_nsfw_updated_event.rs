use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const POST_IS_NSFW_UPDATED_EVENT_TYPE: &str = "dona.post_is_nsfw_updated";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostIsNsfwUpdatedEvent {
    id: String,
    user_id: String,
    is_nsfw: bool,
    updated_at: String,

    base_event: BaseEvent,
}

impl PostIsNsfwUpdatedEvent {
    pub fn new(id: String, user_id: String, is_nsfw: bool, updated_at: String) -> Self {
        Self {
            id: id.clone(),
            user_id,
            is_nsfw,
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

    pub fn is_nsfw(&self) -> bool {
        self.is_nsfw
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for PostIsNsfwUpdatedEvent {
    fn event_type(&self) -> &'static str {
        POST_IS_NSFW_UPDATED_EVENT_TYPE
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
                ("is_nsfw".to_string(), self.is_nsfw.to_string()),
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
            .ok_or(EventDeserializeError::MissingField("id".to_string()))?
            .clone();
        let user_id = data
            .get("user_id")
            .ok_or(EventDeserializeError::MissingField("user_id".to_string()))?
            .clone();
        let is_nsfw = data
            .get("is_nsfw")
            .ok_or(EventDeserializeError::MissingField("is_nsfw".to_string()))?
            .parse()
            .map_err(|_| EventDeserializeError::InvalidField("is_nsfw".to_string()))?;
        let updated_at = data
            .get("updated_at")
            .ok_or(EventDeserializeError::MissingField(
                "updated_at".to_string(),
            ))?
            .clone();

        Ok(Box::new(Self {
            id,
            user_id,
            is_nsfw,
            updated_at,
            base_event,
        }))
    }
}
