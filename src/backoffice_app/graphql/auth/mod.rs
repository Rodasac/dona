use async_graphql::MergedObject;

use self::{create_mutation::CreateUserMutation, update_mutation::UpdateUserMutation};

mod create_mutation;
mod update_mutation;

#[derive(MergedObject, Default)]
pub struct AuthMutation(CreateUserMutation, UpdateUserMutation);
