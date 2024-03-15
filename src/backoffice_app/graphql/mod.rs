use async_graphql::MergedObject;

use self::auth::{AuthMutation, AuthQuery};

mod auth;

#[derive(MergedObject, Default)]
pub struct BackofficeQuery(AuthQuery);

#[derive(MergedObject, Default)]
pub struct BackofficeMutation(AuthMutation);
