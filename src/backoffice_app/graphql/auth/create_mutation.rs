use async_graphql::{Context, Error, InputObject, Object, Result, Upload};
use backoffice::auth::application::create_user::command::CreateUserCommand;
use poem::session::Session;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::{
    gql_validators::{check_admin, check_upload, is_authenticated},
    CommandBusType,
};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub id: Uuid,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub username: String,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: String,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub full_name: String,
    pub profile_picture: Option<Upload>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct CreateUserMutation;

#[Object]
impl CreateUserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<bool> {
        let command_bus = ctx.data::<CommandBusType>()?;

        let session = ctx.data::<Session>()?;
        if is_authenticated(session) {
            check_admin(command_bus, session).await?;
        }

        let upload_value = input
            .profile_picture
            .clone()
            .map(|p| p.value(ctx))
            .transpose()?;
        check_upload(&upload_value)?;

        let (profile_name, profile_file) = if let Some(upload_value) = &upload_value {
            (
                Some(upload_value.filename.clone()),
                Some(upload_value.content.try_clone()?),
            )
        } else {
            (None, None)
        };

        let command = CreateUserCommand {
            id: input.id.to_string(),
            username: input.username,
            email: input.email,
            password: input.password,
            full_name: input.full_name,
            profile_picture: profile_name,
            profile_picture_file: profile_file,
            is_admin: false,
            created_at: input.created_at.format(&Rfc3339)?,
            updated_at: input.updated_at.format(&Rfc3339)?,
        };
        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
