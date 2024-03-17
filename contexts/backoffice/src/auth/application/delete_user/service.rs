use std::sync::Arc;

use crate::auth::domain::{user::UserId, user_repository::UserRepository};

#[derive(Clone)]
pub struct UserDeleter {
    user_repository: Arc<dyn UserRepository>,
}

impl UserDeleter {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: UserId) -> Result<(), String> {
        self.user_repository
            .delete(id)
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::auth::domain::user::tests::UserIdMother;
    use crate::auth::domain::user_repository::tests::MockUserRepository;
    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;

    #[tokio::test]
    async fn should_return_error_when_user_not_found() {
        let user_id = UserIdMother::random();
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_delete()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(|_| {
                Err(BaseRepositoryError::UnexpectedError(
                    "User not found".to_string(),
                ))
            });

        let user_deleter = UserDeleter::new(Arc::new(user_repository));

        let result = user_deleter.execute(user_id).await;

        assert_eq!(result, Err("Unexpected error: User not found".to_string()));
    }

    #[tokio::test]
    async fn should_delete_user() {
        let user_id = UserIdMother::random();
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_delete()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let user_deleter = UserDeleter::new(Arc::new(user_repository));

        let result = user_deleter.execute(user_id).await;

        assert!(result.is_ok());
    }
}
