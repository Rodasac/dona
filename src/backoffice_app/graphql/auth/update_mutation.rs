use async_graphql::{Context, Error, InputObject, Object, Result, Upload};
use backoffice::auth::application::update_user::command::UpdateUserCommand;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::{gql_validators::check_upload, CommandBusType};

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: Uuid,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub username: Option<String>,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: Option<String>,
    #[graphql(validator(chars_min_length = 1, chars_max_length = 150))]
    pub full_name: Option<String>,
    pub profile_picture: Option<Option<Upload>>,
    pub is_admin: Option<bool>,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default)]
pub struct UpdateUserMutation;

#[Object]
impl UpdateUserMutation {
    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<bool> {
        let upload_value = input
            .profile_picture
            .clone()
            .map(|p| p.map(|p| p.value(ctx)).transpose())
            .transpose()?;
        if let Some(value) = &upload_value {
            check_upload(value)?;
        }

        let (profile_name, profile_file) = if let Some(upload_value) = &upload_value {
            if let Some(upload_value) = upload_value {
                (
                    Some(Some(upload_value.filename.clone())),
                    Some(upload_value.content.try_clone()?),
                )
            } else {
                (Some(None), None)
            }
        } else {
            (None, None)
        };

        let command = UpdateUserCommand {
            id: input.id.to_string(),
            username: input.username,
            password: input.password,
            full_name: input.full_name,
            is_admin: input.is_admin,
            profile_picture: profile_name,
            profile_picture_file: profile_file,
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
