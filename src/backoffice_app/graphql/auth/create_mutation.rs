use async_graphql::{Context, Error, InputObject, Object, Result, Upload};
use backoffice::auth::application::create_user::command::CreateUserCommand;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::{CommandBusType, MAX_UPLOAD_SIZE};

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
    pub is_admin: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct CreateUserMutation;

#[Object]
impl CreateUserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<bool> {
        let upload_value = input
            .profile_picture
            .clone()
            .map(|p| p.value(ctx))
            .transpose()?;
        if let Some(upload_value) = &upload_value {
            if upload_value.size()? > MAX_UPLOAD_SIZE {
                return Err(Error::new("File too large"));
            }
        }

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
            is_admin: input.is_admin,
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
