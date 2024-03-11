use std::sync::Arc;

use crate::auth::domain::{
    password_hasher::UserPasswordHasher,
    user::{User, UserCreatedAt, UserEmail, UserFullName, UserId, UserPassword, UserUpdatedAt},
    user_repository::UserRepository,
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
        id: UserId,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> Result<(), String> {
        let hashed_password = self
            .password_hasher
            .hash(&password.to_string())
            .map(|hashed_password| UserPassword::new(hashed_password).unwrap())
            .map_err(|e| e.to_string())?;

        let user = User::new_user(
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
    use crate::auth::domain::user::tests::{
        UserCreatedAtMother, UserEmailMother, UserFullNameMother, UserIdMother, UserPasswordMother,
        UserUpdatedAtMother,
    };
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
                UserIdMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
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
                UserIdMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
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
                UserIdMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
            )
            .await;

        assert_eq!(result, Ok(()));
    }
}
