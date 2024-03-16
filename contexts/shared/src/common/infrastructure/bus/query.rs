use std::{collections::HashMap, sync::Arc};

use crate::common::domain::bus::query::{Query, QueryBus, QueryError, QueryHandler, Response};

#[derive(Clone, Default)]
pub struct InMemoryQueryBus {
    handlers: HashMap<&'static str, Arc<dyn QueryHandler>>,
}

impl InMemoryQueryBus {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl QueryBus for InMemoryQueryBus {
    async fn ask(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query_type = query.query_type();
        let handler = self.handlers.get(query_type).ok_or_else(|| {
            QueryError::new(format!("No handler found for query: {}", query_type))
        })?;
        handler.handle(query).await
    }

    fn register_handler(&mut self, query_type: &'static str, handler: Arc<dyn QueryHandler>) {
        self.handlers.insert(query_type, handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestQuery;

    impl Query for TestQuery {
        fn query_type(&self) -> &'static str {
            "TestQuery"
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestQueryHandler;

    #[async_trait::async_trait]
    impl QueryHandler for TestQueryHandler {
        async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
            let query = query
                .as_any()
                .downcast_ref::<TestQuery>()
                .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

            assert_eq!(query, &TestQuery);

            Ok(Box::new(TestResponse))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestResponse;

    impl Response for TestResponse {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[tokio::test]
    async fn test_in_memory_query_bus() {
        let mut bus = InMemoryQueryBus::new();
        let handler = Arc::new(TestQueryHandler);
        bus.register_handler("TestQuery", handler);

        let query = Box::new(TestQuery);
        let response = bus.ask(query).await.unwrap();
        let response = response.as_any().downcast_ref::<TestResponse>().unwrap();
        assert_eq!(response, &TestResponse);
    }
}
