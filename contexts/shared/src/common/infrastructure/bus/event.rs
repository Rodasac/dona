use std::{collections::HashMap, sync::Arc};

use crate::common::domain::bus::event::{Event, EventBus, EventError, EventHandler};

#[derive(Clone, Default)]
pub struct InMemoryEventBus {
    handlers: HashMap<&'static str, Vec<Arc<dyn EventHandler>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: Vec<Arc<dyn Event>>) -> Result<(), EventError> {
        for e in event {
            let event_type = e.event_type();
            if let Some(handlers) = self.handlers.get(event_type) {
                for handler in handlers {
                    handler.handle(e.clone()).await?;
                }
            }
        }

        Ok(())
    }

    fn register_handler(&mut self, handler: Arc<dyn EventHandler>) {
        for event_type in handler.subscribed_to() {
            self.handlers
                .entry(event_type)
                .and_modify(|handlers| handlers.push(handler.clone()))
                .or_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::domain::bus::event::{EventDeserializeError, EventSerialized};

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEvent;

    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "TestEvent"
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn from_primitives(
            &self,
            _primitives: EventSerialized,
        ) -> Result<Box<dyn Event>, EventDeserializeError> {
            Ok(Box::new(TestEvent))
        }

        fn to_primitives(&self) -> EventSerialized {
            EventSerialized::default()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEventHandler;

    #[async_trait::async_trait]
    impl EventHandler for TestEventHandler {
        async fn handle(&self, event: Arc<dyn Event>) -> Result<(), EventError> {
            let test_event = event.as_any().downcast_ref::<TestEvent>().unwrap();
            assert_eq!(test_event, &TestEvent);

            Ok(())
        }

        fn subscribed_to(&self) -> Vec<&'static str> {
            vec!["TestEvent"]
        }
    }

    #[tokio::test]
    async fn test_publish() {
        let mut bus = InMemoryEventBus::new();
        let handler = Arc::new(TestEventHandler);
        bus.register_handler(handler.clone());

        let event = Arc::new(TestEvent);
        bus.publish(vec![event.clone()]).await.unwrap();
    }
}
