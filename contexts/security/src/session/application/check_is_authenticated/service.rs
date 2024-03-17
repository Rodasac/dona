use crate::session::domain::repository::SessionRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct IsAuthenticatedChecker {
    session_repository: Arc<dyn SessionRepository>,
}

impl IsAuthenticatedChecker {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn execute(&self, session_id: String) -> Result<(), String> {
        let _ = self
            .session_repository
            .get(&session_id)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use shared::domain::base_errors::BaseRepositoryError;
    use time::OffsetDateTime;

    use crate::session::domain::{repository::tests::MockSessionRepository, user::UserSession};

    use super::*;
    #[tokio::test]
    async fn should_return_ok_if_session_exists() {
        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            false,
        );
        let mut session_repo = MockSessionRepository::new();
        session_repo
            .expect_get()
            .withf(|session_id| session_id == "session_id")
            .returning(move |_| Ok(session.clone()));

        let permission_checker = IsAuthenticatedChecker::new(Arc::new(session_repo));
        let result = permission_checker.execute("session_id".to_string()).await;

        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn should_return_err_if_session_does_not_exist() {
        let mut session_repo = MockSessionRepository::new();
        session_repo
            .expect_get()
            .withf(|session_id| session_id == "session_id")
            .returning(|_| {
                Err(BaseRepositoryError::UnexpectedError(
                    "Session not found".to_string(),
                ))
            });

        let permission_checker = IsAuthenticatedChecker::new(Arc::new(session_repo));
        let result = permission_checker.execute("session_id".to_string()).await;

        assert_eq!(
            result,
            Err("Unexpected error: Session not found".to_string())
        );
    }
}
