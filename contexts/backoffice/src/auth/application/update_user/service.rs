use std::{fs::File, sync::Arc};

use shared::{
    check_file_extension,
    domain::{storage::FileStorageRepository, value_objects::user_id::UserId},
};

use crate::auth::domain::{
    password_hasher::UserPasswordHasher,
    user::{
        User, UserFullName, UserIsAdmin, UserPassword, UserProfilePicture, UserUpdatedAt,
        UserUsername,
    },
    user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UpdateUser {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn UserPasswordHasher>,
    storage_repository: Arc<dyn FileStorageRepository>,
}

impl UpdateUser {
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
        username: Option<UserUsername>,
        password: Option<UserPassword>,
        fullname: Option<UserFullName>,
        profile_picture: Option<UserProfilePicture>,
        profile_picture_file: Option<File>,
        is_admin: Option<UserIsAdmin>,
        updated_at: UserUpdatedAt,
    ) -> Result<User, String> {
        let mut user = self.user_id_exists(id.clone()).await?;

        let password = match password {
            Some(password) => Some(
                self.password_hasher
                    .hash(&password.to_string())
                    .map(|hash| UserPassword::new(hash).unwrap())
                    .map_err(|e| e.to_string())?,
            ),
            None => None,
        };

        if let Some(profile_picture) = profile_picture.clone() {
            if let Some(profile_picture) = profile_picture.value() {
                check_file_extension(&profile_picture.to_string())?;

                let profile_picture_file =
                    profile_picture_file.ok_or("File not present".to_string())?;

                self.storage_repository
                    .save(
                        "user".to_string(),
                        id.to_string(),
                        profile_picture.to_string(),
                        profile_picture_file,
                    )
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        user.update(
            username,
            password,
            fullname,
            profile_picture,
            is_admin,
            updated_at,
        )?;

        self.user_repository
            .save(&user)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
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
        UserFullNameMother, UserIsAdminMother, UserMother, UserPasswordMother,
        UserProfilePictureMother, UserUpdatedAtMother, UserUsernameMother,
    };
    use crate::auth::domain::user_repository::tests::MockUserRepository;

    #[tokio::test]
    async fn should_fail_user_not_found() {
        let user = UserMother::random();
        let user_id = UserId::new(user.id().to_string()).unwrap();
        let username = UserUsernameMother::random();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let profile_picture = UserProfilePictureMother::random();
        let profile_picture_file = None;
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(move |_| Err(BaseRepositoryError::NotFound));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(MockUserPasswordHasher::new()),
            Arc::new(MockFileStorageRepository::new()),
        );

        let result = service
            .execute(
                user_id,
                Some(username),
                Some(password),
                Some(fullname),
                Some(profile_picture),
                profile_picture_file,
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_fail_password_hasher() {
        let user = UserMother::random();
        let user_id = UserId::new(user.id().to_string()).unwrap();
        let username = UserUsernameMother::random();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let profile_picture = UserProfilePictureMother::random();
        let profile_picture_file = None;
        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Err(HashError::InvalidPassword));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(MockFileStorageRepository::new()),
        );

        let result = service
            .execute(
                user_id,
                Some(username),
                Some(password),
                Some(fullname),
                Some(profile_picture),
                profile_picture_file,
                Some(is_admin),
                updated_at,
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_fail_profile_image_file_save() {
        let user = UserMother::random();
        let user_id = UserId::new(user.id().to_string()).unwrap();
        let username = UserUsernameMother::random();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let profile_picture = UserProfilePictureMother::random();

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();
        let profile_picture_file = Some(image);

        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storage_repository = MockFileStorageRepository::new();
        storage_repository
            .expect_save()
            .times(1)
            .return_const(Err("Error saving the image".to_string()));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storage_repository),
        );

        let result = service
            .execute(
                user_id,
                Some(username),
                Some(password),
                Some(fullname),
                Some(profile_picture),
                profile_picture_file,
                Some(is_admin),
                updated_at,
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_fail_user_update_repo() {
        let user = UserMother::random();
        let user_id = UserId::new(user.id().to_string()).unwrap();
        let username = UserUsernameMother::random();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let profile_picture = UserProfilePictureMother::random();

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();
        let profile_picture_file = Some(image);

        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storage_repository = MockFileStorageRepository::new();
        storage_repository
            .expect_save()
            .times(1)
            .return_const(Ok("".to_string()));

        user_repository
            .expect_save()
            .times(1)
            .returning(|_| Err(BaseRepositoryError::UnexpectedError("DB error".to_string())));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storage_repository),
        );

        let result = service
            .execute(
                user_id,
                Some(username),
                Some(password),
                Some(fullname),
                Some(profile_picture),
                profile_picture_file,
                Some(is_admin),
                updated_at,
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_update_user() {
        let user = UserMother::random();
        let user_id = UserId::new(user.id().to_string()).unwrap();
        let username = UserUsernameMother::random();
        let password = UserPasswordMother::random();
        let fullname = UserFullNameMother::random();
        let profile_picture = UserProfilePictureMother::random();

        let file_rng_path = format!("{}.jpg", Uuid::new_v4());
        let mut image = File::create(file_rng_path.clone()).unwrap();
        image.write_all(b"test").unwrap();
        image.seek(std::io::SeekFrom::Start(0)).unwrap();
        let profile_picture_file = Some(image);

        let is_admin = UserIsAdminMother::inverted(user.is_admin());
        let updated_at = UserUpdatedAtMother::random_after(user.updated_at());

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        user_repository.expect_save().times(1).returning(|_| Ok(()));

        let mut password_hasher = MockUserPasswordHasher::new();
        password_hasher
            .expect_hash()
            .with(predicate::eq(password.to_string()))
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut storage_repository = MockFileStorageRepository::new();
        storage_repository
            .expect_save()
            .times(1)
            .return_const(Ok("".to_string()));

        let service = UpdateUser::new(
            Arc::new(user_repository),
            Arc::new(password_hasher),
            Arc::new(storage_repository),
        );

        let result = service
            .execute(
                user_id,
                Some(username),
                Some(password),
                Some(fullname),
                Some(profile_picture),
                profile_picture_file,
                Some(is_admin),
                updated_at,
            )
            .await;

        std::fs::remove_file(file_rng_path).unwrap();
        assert!(result.is_ok());
    }
}
