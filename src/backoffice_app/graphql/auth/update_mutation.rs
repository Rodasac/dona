use async_graphql::{Context, Error, InputObject, Object, Result};
use backoffice::auth::application::update_user::command::UpdateUserCommand;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use uuid::Uuid;

use crate::backoffice_app::di::BackofficeCommandBusType;

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: Option<String>,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub full_name: Option<String>,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct UpdateUserMutation;

#[Object]
impl UpdateUserMutation {
    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<bool> {
        let command = UpdateUserCommand {
            id: input.id.to_string(),
            password: input.password,
            full_name: input.full_name,
            updated_at: input.updated_at.format(&Iso8601::DEFAULT)?,
        };

        let command_bus = ctx.data::<BackofficeCommandBusType>()?;
        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
