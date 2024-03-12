use async_graphql::MergedObject;

use self::auth::AuthMutation;

mod auth;

#[derive(MergedObject, Default)]
pub struct BackofficeMutation(AuthMutation);
