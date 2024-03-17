use shared::domain::bus::event::{BaseEvent, Event, EventDeserializeError, EventSerialized};

pub const USER_CREATED_EVENT_TYPE: &str = "auth.user_created";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserCreatedEvent {
    id: String,
    email: String,
    password: String,
    full_name: String,
    is_admin: bool,
    created_at: String,
    updated_at: String,

    base_event: BaseEvent,
}

impl UserCreatedEvent {
    pub fn new(
        id: String,
        email: String,
        password: String,
        full_name: String,
        is_admin: bool,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id: id.clone(),
            email,
            password,
            full_name,
            is_admin,
            created_at,
            updated_at,
            base_event: BaseEvent::new(id),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

impl Event for UserCreatedEvent {
    fn event_type(&self) -> &'static str {
        USER_CREATED_EVENT_TYPE
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
                ("email".to_string(), self.email.clone()),
                ("password".to_string(), self.password.clone()),
                ("full_name".to_string(), self.full_name.clone()),
                ("is_admin".to_string(), self.is_admin.to_string()),
                ("created_at".to_string(), self.created_at.clone()),
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
        let email = data
            .get("email")
            .ok_or(EventDeserializeError::MissingField("email".to_string()))?;
        let password = data
            .get("password")
            .ok_or(EventDeserializeError::MissingField("password".to_string()))?;
        let full_name = data
            .get("full_name")
            .ok_or(EventDeserializeError::MissingField("full_name".to_string()))?;
        let is_admin = data
            .get("is_admin")
            .ok_or(EventDeserializeError::MissingField("is_admin".to_string()))?
            .parse::<bool>()
            .map_err(|_| EventDeserializeError::InvalidField("is_admin".to_string()))?;
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

        Ok(Box::new(UserCreatedEvent {
            id: id.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
            full_name: full_name.to_owned(),
            is_admin,
            created_at: created_at.to_owned(),
            updated_at: updated_at.to_owned(),
            base_event,
        }))
    }
}
