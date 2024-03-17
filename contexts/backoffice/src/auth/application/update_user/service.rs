use std::sync::Arc;

use crate::auth::domain::{
    password_hasher::UserPasswordHasher,
    user::{User, UserFullName, UserId, UserIsAdmin, UserPassword, UserUpdatedAt},
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UpdateUser {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn UserPasswordHasher>,
}

impl UpdateUser {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn UserPasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    async fn user_id_exists(&self, id: UserId) -> Result<User, String> {
        let user = self.user_repository.find_by_id(id).await;

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err("User not found".to_string()),
        }
    }

    pub async fn execute(
        &self,
        id: UserId,
        password: Option<UserPassword>,
        fullname: Option<UserFullName>,
        is_admin: Option<UserIsAdmin>,
        updated_at: UserUpdatedAt,
    ) -> Result<User, String> {
        let mut user = self.user_id_exists(id).await?;

        let password = match password {
            Some(password) => Some(
                self.password_hasher
                    .hash(&password.to_string())
                    .map(|hash| UserPassword::new(hash).unwrap())
                    .map_err(|e| e.to_string())?,
            ),
            None => None,
        };

        user.update(password, fullname, is_admin, updated_at)?;

        self.user_repository
            .save(&user)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;

    use crate::auth::domain::password_hasher::tests::MockUserPasswordHasher;
    use crate::auth::domain::password_hasher::HashError;
    use crate::auth::domain::user::tests::{
        UserFullNameMother, UserIsAdminMother, UserMother, UserPasswordMother, UserUpdatedAtMother,
    };
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn test_update_user_fails_user_not_found() {
        let user = UserMother::random();
        let id = user.id().clone();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after_updated(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(move |_| Err(BaseRepositoryError::NotFound));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(MockUserPasswordHasher::new()),
        );

        let result = service
            .execute(
                id,
                Some(password),
                Some(fullname),
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_fails_password_hasher_fails() {
        let user = UserMother::random();
        let id = user.id().clone();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after_updated(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Err(HashError::InvalidPassword));

        let service = UpdateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = service
            .execute(
                id,
                Some(password),
                Some(fullname),
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_fails_user_update_fails() {
        let user = UserMother::random();
        let id = user.id().clone();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after_updated(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        user_repository
            .expect_save()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::UnexpectedError("DB error".to_string())));

        let service = UpdateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = service
            .execute(
                id,
                Some(password),
                Some(fullname),
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_should_update_user() {
        let user = UserMother::random();
        let id = user.id().clone();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after_updated(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        user_repository.expect_save().times(1).returning(|_| Ok(()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let service = UpdateUser::new(Arc::new(user_repository), Arc::new(password_hasher));

        let result = service
            .execute(
                id,
                Some(password),
                Some(fullname),
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_ok());
    }
}
