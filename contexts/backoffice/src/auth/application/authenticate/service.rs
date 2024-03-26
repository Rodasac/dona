use std::sync::Arc;

use shared::domain::criteria::{
    cursor::{Cursor, FirstField},
    filter::{Filter, FilterField, FilterOperator, FilterValue},
    Criteria,
};

use crate::auth::domain::{
    password_hasher::UserPasswordHasher,
    user::{User, UserEmail, UserPassword},
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserAuthenticator {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn UserPasswordHasher>,
}

impl UserAuthenticator {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn UserPasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    async fn find_by_email(&self, email: UserEmail) -> Result<User, String> {
        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("email".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from(email.to_string()).unwrap(),
            )],
            None,
            Some(Cursor::new(
                None,
                None,
                Some(FirstField::new(1).unwrap()),
                None,
            )),
        );

        let user = self
            .user_repository
            .find_by_criteria(criteria)
            .await
            .map_err(|e| e.to_string())?;

        if user.is_empty() {
            return Err("Not found".to_string());
        }

        Ok(user.first().unwrap().to_owned())
    }

    pub async fn execute(&self, email: UserEmail, password: UserPassword) -> Result<(), String> {
        let user = self.find_by_email(email).await?;

        self.password_hasher
            .verify(
                password.to_string().as_str(),
                user.password().to_string().as_str(),
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use shared::domain::base_errors::BaseRepositoryError;

    use super::*;

    use crate::auth::domain::password_hasher::tests::MockUserPasswordHasher;
    use crate::auth::domain::password_hasher::HashError;
    use crate::auth::domain::user::tests::{UserEmailMother, UserMother, UserPasswordMother};
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn should_return_error_when_user_not_found() {
        let user_email = UserEmailMother::random();
        let user_password = UserPasswordMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .return_const(Err(BaseRepositoryError::NotFound));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_verify()
            .times(0)
            .return_const(Ok(()));

        let user_auth =
            UserAuthenticator::new(Arc::new(user_repository), Arc::new(password_hasher));
        let result = user_auth
            .execute(user_email.clone(), user_password.clone())
            .await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Not found");
    }

    #[tokio::test]
    async fn should_return_error_when_password_is_incorrect() {
        let user = UserMother::random();
        let user_password = UserPasswordMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .return_const(Ok(vec![user.clone()]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_verify()
            .times(1)
            .return_const(Err(HashError::InvalidPassword));

        let user_auth =
            UserAuthenticator::new(Arc::new(user_repository), Arc::new(password_hasher));
        let result = user_auth
            .execute(
                UserEmail::new(user.email().to_owned()).unwrap(),
                user_password.clone(),
            )
            .await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Hash error: Invalid password");
    }

    #[tokio::test]
    async fn should_return_ok_on_correct_password() {
        let user = UserMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .return_const(Ok(vec![user.clone()]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_verify()
            .times(1)
            .return_const(Ok(()));

        let user_auth =
            UserAuthenticator::new(Arc::new(user_repository), Arc::new(password_hasher));
        let response = user_auth
            .execute(
                UserEmail::new(user.email().to_owned()).unwrap(),
                UserPassword::new(user.password().to_owned()).unwrap(),
            )
            .await;

        assert!(response.is_ok());
    }
}
