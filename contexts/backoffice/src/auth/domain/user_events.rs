use serde::{Deserialize, Serialize};
use shared::common::domain::bus::event::{BaseEvent, Event};

pub const USER_CREATED_EVENT_TYPE: &str = "user_created";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    id: String,
    email: String,
    password: String,
    full_name: String,
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
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id: id.clone(),
            email,
            password,
            full_name,
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
}
