use serde::{Deserialize, Serialize};
use shared::domain::{
    bus::query::{Query, QueryError, QueryHandler, Response},
    value_objects::user_id::UserId,
};

use super::service::UserFinder;

pub const FIND_USER_BY_ID_QUERY_TYPE: &str = "auth.find_user_by_id.query";

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FindUserByIdQuery {
    pub id: String,
}

impl Query for FindUserByIdQuery {
    fn query_type(&self) -> &'static str {
        FIND_USER_BY_ID_QUERY_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct FindUserByIdQueryHandler {
    service: UserFinder,
}

impl FindUserByIdQueryHandler {
    pub fn new(service: UserFinder) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl QueryHandler for FindUserByIdQueryHandler {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query = query
            .as_any()
            .downcast_ref::<FindUserByIdQuery>()
            .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

        let user_id =
            UserId::new(query.id.to_owned()).map_err(|e| QueryError::new(e.to_string()))?;

        let user = self
            .service
            .execute(user_id)
            .await
            .map_err(|e| QueryError::new(e.to_string()))?;

        Ok(Box::new(user))
    }
}
