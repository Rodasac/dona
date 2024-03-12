use shared::common::domain::base_errors::BaseRepositoryError;

use super::user::{User, UserEmail, UserId};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<User, BaseRepositoryError>;
    async fn find_by_email(&self, email: UserEmail) -> Result<User, BaseRepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, BaseRepositoryError>;
    async fn save(&self, user: &User) -> Result<(), BaseRepositoryError>;
    async fn delete(&self, id: UserId) -> Result<(), BaseRepositoryError>;
}

#[cfg(test)]
pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub UserRepository {}

        #[async_trait::async_trait]
        impl UserRepository for UserRepository {
            async fn find_by_id(&self, id: UserId) -> Result<User, BaseRepositoryError>;
            async fn find_by_email(&self, email: UserEmail) -> Result<User, BaseRepositoryError>;
            async fn find_all(&self) -> Result<Vec<User>, BaseRepositoryError>;
            async fn save(&self, user: &User) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, id: UserId) -> Result<(), BaseRepositoryError>;
        }
    }
}
