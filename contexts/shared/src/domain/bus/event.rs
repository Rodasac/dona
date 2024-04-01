use std::{
    any::Any,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventDeserializeError {
    MissingField(String),
    InvalidField(String),
}

impl Error for EventDeserializeError {}
impl Display for EventDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "Missing field: {}", field),
            Self::InvalidField(field) => write!(f, "Invalid field: {}", field),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSerialized {
    event_id: String,
    aggregate_id: String,
    occurred_at: String,
    data: HashMap<String, String>,
}

impl EventSerialized {
    pub fn new(
        event_id: String,
        aggregate_id: String,
        occurred_at: String,
        data: HashMap<String, String>,
    ) -> Self {
        Self {
            event_id,
            aggregate_id,
            occurred_at,
            data,
        }
    }

    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }

    pub fn occurred_at(&self) -> &str {
        &self.occurred_at
    }

    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }
}

pub trait Event: Debug + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;

    fn to_primitives(&self) -> EventSerialized;
    #[allow(clippy::wrong_self_convention)]
    fn from_primitives(
        &self,
        primitives: EventSerialized,
    ) -> Result<Box<dyn Event>, EventDeserializeError>;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseEvent {
    event_id: String,
    aggregate_id: String,
    occurred_at: String,
}

impl BaseEvent {
    pub fn new(aggregate_id: String) -> Self {
        Self {
            event_id: Uuid::now_v7().to_string(),
            aggregate_id,
            occurred_at: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
        }
    }

    pub fn from_primitives(event_id: String, aggregate_id: String, occurred_at: String) -> Self {
        Self {
            event_id,
            aggregate_id,
            occurred_at,
        }
    }

    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    pub fn aggregate_id(&self) -> &str {
        &self.aggregate_id
    }

    pub fn occurred_at(&self) -> &str {
        &self.occurred_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventError(String);

impl EventError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl Error for EventError {}
impl Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventError: {}", self.0)
    }
}

impl From<String> for EventError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<EventError> for String {
    fn from(error: EventError) -> Self {
        error.0
    }
}

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: Arc<dyn Event>) -> Result<(), EventError>;
    fn subscribed_to(&self) -> Vec<&'static str>;
}

#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: Vec<Arc<dyn Event>>) -> Result<(), EventError>;
    fn register_handler(&mut self, handler: Arc<dyn EventHandler>);
}

pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub EventBus {}

        #[async_trait::async_trait]
        impl EventBus for EventBus {
            async fn publish(&self, event: Vec<Arc<dyn Event>>) -> Result<(), EventError>;
            fn register_handler(&mut self, handler: Arc<dyn EventHandler>);
        }
    }
}
