use std::vec;

use async_graphql::{Context, Error, InputObject, Object, Result};
use backoffice::auth::application::{
    authenticate::command::AuthenticateUserCommand,
    find_users_by_criteria::query::FindUsersByCriteriaQuery, response::UsersResponse,
};
use poem::session::Session;
use security::session::application::create::command::CreateSessionCommand;
use shared::domain::criteria::{
    cursor::{Cursor, FirstField},
    filter::{Filter, FilterField, FilterOperator, FilterValue},
    Criteria,
};
use uuid::Uuid;

use crate::{CommandBusType, QueryBusType};

#[derive(InputObject)]
pub struct LoginInput {
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(chars_min_length = 8, chars_max_length = 50))]
    pub password: String,
}

#[derive(Debug, Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let session = ctx.data::<Session>()?;

        let user_id = session.get::<String>("user_id");
        if user_id.is_some() {
            return Ok(true);
        }

        let command = AuthenticateUserCommand {
            email: input.email.clone(),
            password: input.password,
        };

        let command_bus = ctx.data::<CommandBusType>()?;
        command_bus
            .dispatch(Box::new(command))
            .await
            .map_err(|_| Error::new("INVALID_CREDENTIALS"))?;

        let user_query = FindUsersByCriteriaQuery {
            criteria: Criteria::new(
                vec![Filter::new(
                    FilterField::try_from("email".to_string()).unwrap(),
                    FilterOperator::Equal,
                    FilterValue::try_from(input.email).unwrap(),
                )],
                None,
                Some(Cursor::new(
                    None,
                    None,
                    Some(FirstField::new(1).unwrap()),
                    None,
                )),
            ),
        };
        let query_bus = ctx.data::<QueryBusType>()?;
        let user_resp = query_bus
            .ask(Box::new(user_query))
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        let user_resp = user_resp
            .as_any()
            .downcast_ref::<UsersResponse>()
            .ok_or(Error::new("Invalid response".to_string()))?;
        let user = user_resp
            .users
            .first()
            .ok_or(Error::new("User not found".to_string()))?;

        let session_id = Uuid::new_v4().to_string();
        session.set("user_id", user.id.to_string());
        session.set("session_id", session_id.clone());

        let session_command = CreateSessionCommand {
            user_id: user.id.to_string(),
            session_id,
            login_at: time::OffsetDateTime::now_utc(),
            user_is_admin: user.is_admin,
        };
        command_bus
            .dispatch(Box::new(session_command))
            .await
            .map_err(|_| Error::new("UNKNOWN_ERROR"))?;

        Ok(true)
    }
}
