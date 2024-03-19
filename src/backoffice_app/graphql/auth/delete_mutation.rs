use async_graphql::{Context, Error, Object, Result};
use backoffice::auth::application::delete_user::command::DeleteUserCommand;
use poem::session::Session;
use uuid::Uuid;

use crate::{gql_validators::is_authenticated_with_err, CommandBusType};

#[derive(Debug, Default)]
pub struct DeleteUserMutation;

#[Object]
impl DeleteUserMutation {
    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let command = DeleteUserCommand { id: id.to_string() };
        let command_bus = ctx.data::<CommandBusType>()?;

        let session = ctx.data::<Session>()?;
        is_authenticated_with_err(session)?;

        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
