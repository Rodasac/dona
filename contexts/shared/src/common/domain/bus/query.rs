use std::{any::Any, error::Error, fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

pub trait Query: Send + Sync {
    fn query_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryError(String);

impl QueryError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl Error for QueryError {}
impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QueryError: {}", self.0)
    }
}

pub trait Response: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[async_trait::async_trait]
pub trait QueryHandler: Send + Sync {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError>;
}

#[async_trait::async_trait]
pub trait QueryBus: Send + Sync {
    async fn ask(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError>;
    fn register_handler(&mut self, query_type: &'static str, handler: Arc<dyn QueryHandler>);
}
