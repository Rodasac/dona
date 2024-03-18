use std::sync::Arc;

use crate::auth::{
    application::response::UserResponse,
    domain::{user::UserId, user_repository::UserRepository},
};

#[derive(Clone)]
pub struct UserFinder {
    user_repository: Arc<dyn UserRepository>,
}

impl UserFinder {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: UserId) -> Result<UserResponse, String> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(UserResponse {
            id: user.id().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            full_name: user.full_name().to_string(),
            profile_picture: user.profile_picture().value().map(|v| v.to_owned()),
            created_at: user.created_at().to_string(),
            updated_at: user.updated_at().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;

    use super::*;

    use crate::auth::domain::user::tests::{UserIdMother, UserMother};
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn should_return_error_when_user_not_found() {
        let user_id = UserIdMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.to_owned()))
            .return_const(Err(BaseRepositoryError::NotFound));

        let user_finder = UserFinder::new(Arc::new(user_repository));
        let result = user_finder.execute(user_id).await;

        assert_eq!(result, Err("Not found".to_string()));
    }

    #[tokio::test]
    async fn should_return_user_response() {
        let user = UserMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user.id().to_owned()))
            .return_const(Ok(user.clone()));

        let user_finder = UserFinder::new(Arc::new(user_repository));
        let response = user_finder.execute(user.id().to_owned()).await.unwrap();

        assert_eq!(response.id, user.id().to_string());
        assert_eq!(response.username, user.username().to_string());
        assert_eq!(response.email, user.email().to_string());
        assert_eq!(response.full_name, user.full_name().to_string());
        assert_eq!(
            response.profile_picture,
            user.profile_picture().value().map(|v| v.to_owned())
        );
        assert_eq!(response.created_at, user.created_at().to_string());
        assert_eq!(response.updated_at, user.updated_at().to_string());
    }
}
