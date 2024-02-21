use std::sync::Arc;

use crate::auth::domain::{
    password_hasher::UserPasswordHasher, user::User, user_repository::UserRepository,
};

#[derive(Clone)]
pub struct CreateUser {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn UserPasswordHasher>,
}

impl CreateUser {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn UserPasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    pub async fn execute(
        &self,
        id: String,
        email: String,
        password: String,
        full_name: String,
        created_at: String,
        updated_at: String,
    ) -> Result<(), String> {
        let hashed_password = self
            .password_hasher
            .hash(&password)
            .map_err(|e| e.to_string())?;
        let user = User::new(
            id,
            email,
            hashed_password,
            full_name,
            created_at,
            updated_at,
        )?;

        self.user_repository
            .save(&user)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockall::predicate;
    use shared::common::domain::base_errors::BaseRepositoryError;

    use crate::auth::domain::password_hasher::tests::MockUserPasswordHasher;
    use crate::auth::domain::password_hasher::HashError;
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn test_create_user_repository_fails() {
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::AlreadyExists));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let create_user = CreateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = create_user
            .execute(
                "018dc9d0-ef64-7c4c-9317-4574f06a5250".to_string(),
                "test@test.com".to_string(),
                "password".to_string(),
                "Test Name".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
            )
            .await;

        assert_eq!(result, Err(BaseRepositoryError::AlreadyExists.to_string()));
    }

    #[tokio::test]
    async fn test_create_user_password_hasher_fails() {
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save()
            .times(0)
            .returning(|_| Err(BaseRepositoryError::AlreadyExists));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Err(HashError::InvalidPassword));

        let create_user = CreateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = create_user
            .execute(
                "018dc9d0-ef64-7c4c-9317-4574f06a5250".to_string(),
                "test@test.com".to_string(),
                "password".to_string(),
                "Test Name".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
            )
            .await;

        assert_eq!(result, Err(HashError::InvalidPassword.to_string()));
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let mut user_repository = MockUserRepository::new();
        user_repository.expect_save().times(1).returning(|_| Ok(()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let create_user = CreateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = create_user
            .execute(
                "018dc9d0-ef64-7c4c-9317-4574f06a5250".to_string(),
                "test@test.com".to_string(),
                "password".to_string(),
                "Test Name".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
                "2024-02-21T00:00:00+04:00".to_string(),
            )
            .await;

        assert_eq!(result, Ok(()));
    }
}
