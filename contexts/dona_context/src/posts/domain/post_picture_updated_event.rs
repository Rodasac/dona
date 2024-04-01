use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const POST_PICTURE_UPDATED_EVENT_TYPE: &str = "dona.post_picture_updated";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostPictureUpdatedEvent {
    id: String,
    user_id: String,
    picture: Option<String>,
    updated_at: String,

    base_event: BaseEvent,
}

impl PostPictureUpdatedEvent {
    pub fn new(id: String, user_id: String, picture: Option<String>, updated_at: String) -> Self {
        Self {
            id: id.clone(),
            user_id,
            picture,
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

    pub fn picture(&self) -> Option<&str> {
        self.picture.as_deref()
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for PostPictureUpdatedEvent {
    fn event_type(&self) -> &'static str {
        POST_PICTURE_UPDATED_EVENT_TYPE
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
                (
                    "picture".to_string(),
                    self.picture.clone().unwrap_or("".to_string()),
                ),
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
        let picture = data
            .get("picture")
            .map(|v| match v.as_str() {
                "" => None,
                v => Some(v.to_string()),
            })
            .ok_or(EventDeserializeError::MissingField("picture".to_string()))?;
        let updated_at = data
            .get("updated_at")
            .ok_or(EventDeserializeError::MissingField(
                "updated_at".to_string(),
            ))?;

        Ok(Box::new(Self {
            id: id.to_string(),
            user_id: user_id.to_string(),
            picture,
            updated_at: updated_at.to_string(),
            base_event,
        }))
    }
}
