use crate::session::domain::repository::SessionRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionLogout {
    session_repository: Arc<dyn SessionRepository>,
}

impl SessionLogout {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn execute(&self, session_id: String) -> Result<(), String> {
        self.session_repository
            .delete(&session_id)
            .await
            .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use shared::domain::base_errors::BaseRepositoryError;

    use crate::session::domain::repository::tests::MockSessionRepository;

    use super::*;

    #[tokio::test]
    async fn should_delete_session() {
        let mut session_repo = MockSessionRepository::new();
        session_repo
            .expect_delete()
            .withf(|session_id| session_id == "session_id")
            .returning(|_| Ok(()));

        let service = SessionLogout::new(Arc::new(session_repo));

        let result = service.execute("session_id".to_string()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_when_session_cannot_be_deleted() {
        let mut session_repo = MockSessionRepository::new();
        session_repo
            .expect_delete()
            .withf(|session_id| session_id == "session_id")
            .returning(|_| Err(BaseRepositoryError::UnexpectedError("Error".to_string())));

        let service = SessionLogout::new(Arc::new(session_repo));

        let result = service.execute("session_id".to_string()).await;

        assert!(result.is_err());
    }
}
