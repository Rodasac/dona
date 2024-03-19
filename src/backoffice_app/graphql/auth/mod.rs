use async_graphql::MergedObject;

use self::{
    create_mutation::CreateUserMutation, delete_mutation::DeleteUserMutation,
    find_user_query::FindUserQuery, find_users_query::FindUsersQuery,
    login_mutation::LoginMutation, logout_mutation::LogoutMutation,
    update_mutation::UpdateUserMutation,
};

mod create_mutation;
mod delete_mutation;
mod find_user_query;
mod find_users_query;
mod login_mutation;
mod logout_mutation;
pub mod types;
mod update_mutation;

#[derive(MergedObject, Default)]
pub struct AuthQuery(FindUserQuery, FindUsersQuery);

#[derive(MergedObject, Default)]
pub struct AuthMutation(
    CreateUserMutation,
    UpdateUserMutation,
    DeleteUserMutation,
    LoginMutation,
    LogoutMutation,
);
