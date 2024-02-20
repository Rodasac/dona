use std::{any::Any, error::Error, fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

pub trait Event: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
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
