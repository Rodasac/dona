use std::{any::Any, error::Error, fmt::Display, hash::Hash, sync::Arc};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use uuid::Uuid;

pub trait Event: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
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
            occurred_at: OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap(),
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
