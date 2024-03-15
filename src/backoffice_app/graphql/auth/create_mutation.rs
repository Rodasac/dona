use async_graphql::{Context, Error, InputObject, Object, Result};
use backoffice::auth::application::create_user::command::CreateUserCommand;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::backoffice_app::di::CommandBusType;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub id: Uuid,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: String,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub full_name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct CreateUserMutation;

#[Object]
impl CreateUserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<bool> {
        let command = CreateUserCommand {
            id: input.id.to_string(),
            email: input.email,
            password: input.password,
            full_name: input.full_name,
            created_at: input.created_at.format(&Rfc3339)?,
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
