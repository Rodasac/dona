use shared::common::domain::{
    bus::query::{Query, QueryError, QueryHandler, Response},
    criteria::Criteria,
};

use super::service::UsersFinderByCriteria;

pub const FIND_USERS_BY_CRITERIA_QUERY_TYPE: &str = "auth.find_users_by_criteria.query";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FindUsersByCriteriaQuery {
    pub criteria: Criteria,
}

impl Query for FindUsersByCriteriaQuery {
    fn query_type(&self) -> &'static str {
        FIND_USERS_BY_CRITERIA_QUERY_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct FindUsersByCriteriaQueryHandler {
    service: UsersFinderByCriteria,
}

impl FindUsersByCriteriaQueryHandler {
    pub fn new(service: UsersFinderByCriteria) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl QueryHandler for FindUsersByCriteriaQueryHandler {
    async fn handle(&self, query: Box<dyn Query>) -> Result<Box<dyn Response>, QueryError> {
        let query = query
            .as_any()
            .downcast_ref::<FindUsersByCriteriaQuery>()
            .ok_or_else(|| QueryError::new("Invalid query".to_string()))?;

        let users = self
            .service
            .execute(query.criteria.to_owned())
            .await
            .map_err(|e| QueryError::new(e.to_string()))?;

        Ok(Box::new(users))
    }
}
