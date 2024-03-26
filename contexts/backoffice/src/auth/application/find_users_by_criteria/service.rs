use std::sync::Arc;

use shared::domain::criteria::Criteria;

use crate::auth::{
    application::response::{UserResponse, UsersResponse},
    domain::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UsersFinderByCriteria {
    user_repository: Arc<dyn UserRepository>,
}

impl UsersFinderByCriteria {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, criteria: Criteria) -> Result<UsersResponse, String> {
        let users = self
            .user_repository
            .find_by_criteria(criteria)
            .await
            .map_err(|e| e.to_string())?;

        Ok(UsersResponse {
            users: users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id().to_string(),
                    username: user.username().to_string(),
                    email: user.email().to_string(),
                    full_name: user.full_name().to_string(),
                    profile_picture: user.profile_picture().map(|v| v.to_owned()),
                    is_admin: user.is_admin(),
                    created_at: user.created_at_string(),
                    updated_at: user.updated_at_string(),
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;

    use crate::auth::domain::user::tests::UserMother;
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn should_return_empty_when_users_not_found() {
        let criteria = Criteria::new(vec![], None, None);

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_criteria()
            .with(predicate::eq(criteria.to_owned()))
            .return_const(Ok(vec![]));

        let user_finder = UsersFinderByCriteria::new(Arc::new(user_repository));
        let result = user_finder.execute(criteria).await;

        assert_eq!(result, Ok(UsersResponse { users: vec![] }));
    }

    #[tokio::test]
    async fn should_return_users() {
        let criteria = Criteria::new(vec![], None, None);

        let user = UserMother::random();
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_criteria()
            .with(predicate::eq(criteria.to_owned()))
            .return_const(Ok(vec![user.clone()]));

        let user_finder = UsersFinderByCriteria::new(Arc::new(user_repository));
        let result = user_finder.execute(criteria).await;

        assert_eq!(
            result,
            Ok(UsersResponse {
                users: vec![UserResponse {
                    id: user.id().to_string(),
                    username: user.username().to_string(),
                    email: user.email().to_string(),
                    full_name: user.full_name().to_string(),
                    profile_picture: user.profile_picture().map(|v| v.to_owned()),
                    is_admin: user.is_admin(),
                    created_at: user.created_at_string(),
                    updated_at: user.updated_at_string(),
                }]
            })
        );
    }
}
