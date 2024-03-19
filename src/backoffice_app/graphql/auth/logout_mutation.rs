use async_graphql::{Context, Error, Object, Result};
use poem::session::Session;
use security::session::application::logout::command::LogoutSessionCommand;

use crate::{gql_validators::is_authenticated_with_err, CommandBusType};

#[derive(Debug, Default)]
pub struct LogoutMutation;

#[Object]
impl LogoutMutation {
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let command_bus = ctx.data::<CommandBusType>()?;
        let session = ctx.data::<Session>()?;
        is_authenticated_with_err(session)?;

        let session_id = session
            .get::<String>("session_id")
            .ok_or(Error::new("UNAUTHORIZED"))?;

        let command = LogoutSessionCommand {
            session_id: session_id.clone(),
        };

        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|_| Error::new("UNKNOWN_ERROR"))?;

        session.purge();

        Ok(true)
    }
}
