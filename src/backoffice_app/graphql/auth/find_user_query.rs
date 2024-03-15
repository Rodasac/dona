use async_graphql::{Context, Error, Object, Result};
use backoffice::auth::application::{find_user::query::FindUserByIdQuery, response::UserResponse};
use uuid::Uuid;

use crate::backoffice_app::{di::QueryBusType, graphql::auth::types::User};

#[derive(Debug, Default)]
pub struct FindUserQuery;

#[Object]
impl FindUserQuery {
    async fn find_user(&self, ctx: &Context<'_>, id: Uuid) -> Result<User> {
        let query = FindUserByIdQuery { id: id.to_string() };

        let query_bus = ctx.data::<QueryBusType>()?;
        let user = query_bus
            .ask(Box::new(query))
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        let user: UserResponse = user
            .as_any()
            .downcast_ref::<UserResponse>()
            .unwrap()
            .clone();

        Ok(user.into())
    }
}
