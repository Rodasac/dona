use shared::domain::base_errors::BaseRepositoryError;

use super::user::UserSession;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn get_with_user_id(
        &self,
        user_id: &str,
        session_id: &str,
    ) -> Result<UserSession, BaseRepositoryError>;
    async fn get(&self, session_id: &str) -> Result<UserSession, BaseRepositoryError>;
    async fn save(&self, session: UserSession) -> Result<(), BaseRepositoryError>;
    async fn delete(&self, session_id: &str) -> Result<(), BaseRepositoryError>;
    async fn delete_all(&self, user_id: &str) -> Result<(), BaseRepositoryError>;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    mockall::mock! {
        pub SessionRepository {}

        #[async_trait::async_trait]
        impl SessionRepository for SessionRepository {
            async fn get_with_user_id(&self, user_id: &str, session_id: &str) -> Result<UserSession, BaseRepositoryError>;
            async fn get(&self, session_id: &str) -> Result<UserSession, BaseRepositoryError>;
            async fn save(&self, session: UserSession) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, session_id: &str) -> Result<(), BaseRepositoryError>;
            async fn delete_all(&self, user_id: &str) -> Result<(), BaseRepositoryError>;
        }
    }
}
