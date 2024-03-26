use std::{fs::File, sync::Arc};

use shared::{
    check_file_extension,
    domain::{
        criteria::{
            cursor::{Cursor, FirstField},
            filter::{Filter, FilterField, FilterOperator, FilterValue},
            Criteria,
        },
        storage::FileStorageRepository,
    },
    USER_STORAGE_MODEL,
};

use crate::auth::domain::{
    password_hasher::UserPasswordHasher,
    user::{
        User, UserCreatedAt, UserEmail, UserFullName, UserId, UserIsAdmin, UserPassword,
        UserProfilePicture, UserUpdatedAt, UserUsername,
    },
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct CreateUser {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn UserPasswordHasher>,
    storage_repository: Arc<dyn FileStorageRepository>,
}

impl CreateUser {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn UserPasswordHasher>,
        storage_repository: Arc<dyn FileStorageRepository>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
            storage_repository,
        }
    }

    async fn user_id_exists(&self, id: UserId) -> Result<(), String> {
        let user = self.user_repository.find_by_id(id).await;

        if user.is_ok() {
            return Err("User already exists".to_string());
        }

        Ok(())
    }

    async fn user_email_exists(&self, id: UserId, email: UserEmail) -> Result<(), String> {
        let user = self
            .user_repository
            .find_by_criteria(Criteria::new(
                vec![
                    Filter::new(
                        FilterField::try_from("email".to_string()).unwrap(),
                        FilterOperator::Equal,
                        FilterValue::try_from(email.to_string()).unwrap(),
                    ),
                    Filter::new(
                        FilterField::try_from("id".to_string()).unwrap(),
                        FilterOperator::NotEqual,
                        FilterValue::try_from(id.to_string()).unwrap(),
                    ),
                ],
                None,
                Some(Cursor::new(
                    None,
                    None,
                    Some(FirstField::new(1).unwrap()),
                    None,
                )),
            ))
            .await
            .map_err(|e| e.to_string())?;

        if !user.is_empty() {
            return Err("Email already exists".to_string());
        }

        Ok(())
    }

    pub async fn execute(
        &self,
        id: UserId,
        username: UserUsername,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        profile_picture: UserProfilePicture,
        profile_picture_file: Option<File>,
        is_admin: UserIsAdmin,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> Result<(), String> {
        self.user_id_exists(id.clone()).await?;
        self.user_email_exists(id.clone(), email.clone()).await?;

        let hashed_password = self
            .password_hasher
            .hash(&password.to_string())
            .map(|hashed_password| UserPassword::new(hashed_password).unwrap())
            .map_err(|e| e.to_string())?;

        if let Some(profile_picture_file) = profile_picture_file {
            check_file_extension(&profile_picture.to_string())?;

            self.storage_repository
                .save(
                    USER_STORAGE_MODEL.to_string(),
                    id.to_string(),
                    profile_picture.to_string(),
                    profile_picture_file,
                )
                .await?;
        }

        let user = User::new_user(
            id,
            username,
            email,
            hashed_password,
            full_name,
            profile_picture,
            is_admin,
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
    use std::io::{Seek, Write};

    use super::*;

    use mockall::predicate;
    use shared::domain::base_errors::BaseRepositoryError;
    use shared::domain::storage::tests::MockFileStorageRepository;
    use uuid::Uuid;

    use crate::auth::domain::password_hasher::tests::MockUserPasswordHasher;
    use crate::auth::domain::password_hasher::HashError;
    use crate::auth::domain::user::tests::{
        UserCreatedAtMother, UserEmailMother, UserFullNameMother, UserIdMother, UserIsAdminMother,
        UserMother, UserPasswordMother, UserProfilePictureMother, UserUpdatedAtMother,
        UserUsernameMother,
    };
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn should_fail_user_repository_save() {
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::AlreadyExists));
        user_repository
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository
            .expect_save()
            .times(1)
            .return_const(Ok("profile_picture".to_string()));

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserIdMother::random(),
                UserUsernameMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserProfilePictureMother::random(),
                Some(image),
                UserIsAdminMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Err(BaseRepositoryError::AlreadyExists.to_string()));
    }

    #[tokio::test]
    async fn should_fail_password_hasher() {
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save()
            .times(0)
            .returning(|_| Err(BaseRepositoryError::AlreadyExists));
        user_repository
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Err(HashError::InvalidPassword));

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository.expect_save().times(0);

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserIdMother::random(),
                UserUsernameMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserProfilePictureMother::random(),
                Some(image),
                UserIsAdminMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Err(HashError::InvalidPassword.to_string()));
    }

    #[tokio::test]
    async fn should_fail_user_id_exists() {
        let user = UserMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository.expect_save().times(0);
        user_repository
            .expect_find_by_id()
            .times(1)
            .return_const(Ok(user.clone()));
        user_repository.expect_find_by_criteria().times(0);

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher.expect_hash().times(0);

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository.expect_save().times(0);

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserId::new(user.id().to_string()).unwrap(),
                UserUsername::new(user.username().to_string()).unwrap(),
                UserEmail::new(user.email().to_string()).unwrap(),
                UserPassword::new(user.password().to_string()).unwrap(),
                UserFullName::new(user.full_name().to_string()).unwrap(),
                UserProfilePicture::new(user.profile_picture().map(|v| v.to_string())).unwrap(),
                Some(image),
                UserIsAdmin::new(user.is_admin()),
                UserCreatedAt::from_offset(user.created_at().clone()),
                UserUpdatedAt::from_offset(user.updated_at().clone()),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Err("User already exists".to_string()));
    }

    #[tokio::test]
    async fn should_fail_user_email_exists() {
        let user = UserMother::random();

        let mut user_repository = MockUserRepository::new();
        user_repository.expect_save().times(0);
        user_repository
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .return_const(Ok(vec![user.clone()]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher.expect_hash().times(0);

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository.expect_save().times(0);

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserIdMother::random(),
                UserUsername::new(user.username().to_string()).unwrap(),
                UserEmail::new(user.email().to_string()).unwrap(),
                UserPassword::new(user.password().to_string()).unwrap(),
                UserFullName::new(user.full_name().to_string()).unwrap(),
                UserProfilePicture::new(user.profile_picture().map(|v| v.to_string())).unwrap(),
                Some(image),
                UserIsAdmin::new(user.is_admin()),
                UserCreatedAt::from_offset(user.created_at().clone()),
                UserUpdatedAt::from_offset(user.updated_at().clone()),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Err("Email already exists".to_string()));
    }

    #[tokio::test]
    async fn should_fail_file_repository_save() {
        let mut user_repository = MockUserRepository::new();
        user_repository.expect_save().times(0).returning(|_| Ok(()));
        user_repository
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository
            .expect_save()
            .times(1)
            .return_const(Err("Error".to_string()));

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserIdMother::random(),
                UserUsernameMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserProfilePictureMother::random(),
                Some(image),
                UserIsAdminMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Err("Error".to_string()));
    }

    #[tokio::test]
    async fn should_create_user() {
        let mut user_repository = MockUserRepository::new();
        user_repository.expect_save().times(1).returning(|_| Ok(()));
        user_repository
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::NotFound));
        user_repository
            .expect_find_by_criteria()
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq("password"))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storate_repository = MockFileStorageRepository::new();
        storate_repository
            .expect_save()
            .times(1)
            .return_const(Ok("profile_picture".to_string()));

        let create_user = CreateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storate_repository),
        );

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();

        let result = create_user
            .execute(
                UserIdMother::random(),
                UserUsernameMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::create("password".to_string()),
                UserFullNameMother::random(),
                UserProfilePictureMother::random(),
                Some(image),
                UserIsAdminMother::random(),
                UserCreatedAtMother::random(),
                UserUpdatedAtMother::random(),
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert_eq!(result, Ok(()));
    }
}
