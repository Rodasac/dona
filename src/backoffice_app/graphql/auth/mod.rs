use async_graphql::MergedObject;

use self::create_mutation::CreateUserMutation;

mod create_mutation;

#[derive(MergedObject, Default)]
pub struct AuthMutation(CreateUserMutation);
