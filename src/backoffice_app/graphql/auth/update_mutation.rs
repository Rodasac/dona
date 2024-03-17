use async_graphql::{Context, Error, InputObject, Object, Result};
use backoffice::auth::application::update_user::command::UpdateUserCommand;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::CommandBusType;

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: Option<String>,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub full_name: Option<String>,
    pub is_admin: Option<bool>,
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
            is_admin: input.is_admin,
            updated_at: input.updated_at.format(&Rfc3339)?,
        };

        let command_bus = ctx.data::<CommandBusType>()?;
        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
