use async_graphql::MergedObject;

use self::{
    create_mutation::CreateUserMutation, delete_mutation::DeleteUserMutation,
    find_user_query::FindUserQuery, find_users_query::FindUsersQuery,
    update_mutation::UpdateUserMutation,
};

mod create_mutation;
mod delete_mutation;
mod find_user_query;
mod find_users_query;
pub mod types;
mod update_mutation;

#[derive(MergedObject, Default)]
pub struct AuthQuery(FindUserQuery, FindUsersQuery);

#[derive(MergedObject, Default)]
pub struct AuthMutation(CreateUserMutation, UpdateUserMutation, DeleteUserMutation);
