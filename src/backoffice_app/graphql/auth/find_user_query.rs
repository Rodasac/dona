use async_graphql::{Context, Error, Object, Result};
use backoffice::auth::application::{find_user::query::FindUserByIdQuery, response::UserResponse};
use poem::session::Session;
use uuid::Uuid;

use crate::{
    backoffice_app::graphql::auth::types::User, gql_validators::is_authenticated_with_err,
    QueryBusType,
};

#[derive(Debug, Default)]
pub struct FindUserQuery;

#[Object]
impl FindUserQuery {
    async fn find_user(&self, ctx: &Context<'_>, id: Uuid) -> Result<User> {
        let session = ctx.data::<Session>()?;
        is_authenticated_with_err(session)?;

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
