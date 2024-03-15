use async_graphql::{Context, Error, Object, Result};
use backoffice::auth::application::{
    find_users_by_criteria::query::FindUsersByCriteriaQuery, response::UsersResponse,
};
use shared::common::{
    domain::criteria::Criteria, infrastructure::criteria::async_graphql::CriteriaGql,
};

use crate::backoffice_app::{di::QueryBusType, graphql::auth::types::User};

#[derive(Debug, Default)]
pub struct FindUsersQuery;

#[Object]
impl FindUsersQuery {
    async fn find_users(&self, ctx: &Context<'_>, criteria: CriteriaGql) -> Result<Vec<User>> {
        let criteria: Criteria = criteria.try_into()?;

        let query = FindUsersByCriteriaQuery {
            criteria: criteria.to_owned(),
        };

        let query_bus = ctx.data::<QueryBusType>()?;
        let user = query_bus
            .ask(Box::new(query))
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        let users: UsersResponse = user
            .as_any()
            .downcast_ref::<UsersResponse>()
            .unwrap()
            .clone();

        Ok(users.users.into_iter().map(|user| user.into()).collect())
    }
}
