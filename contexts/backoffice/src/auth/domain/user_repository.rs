use shared::domain::{
    base_errors::BaseRepositoryError, criteria::Criteria, value_objects::user_id::UserId,
};

use super::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<User, BaseRepositoryError>;
    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<User>, BaseRepositoryError>;
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
            async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<User>, BaseRepositoryError>;
            async fn find_all(&self) -> Result<Vec<User>, BaseRepositoryError>;
            async fn save(&self, user: &User) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, id: UserId) -> Result<(), BaseRepositoryError>;
        }
    }
}
