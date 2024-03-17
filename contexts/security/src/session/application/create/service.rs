use std::sync::Arc;

use time::OffsetDateTime;

use crate::session::domain::{repository::SessionRepository, user::UserSession};

#[derive(Clone)]
pub struct SessionCreator {
    session_repository: Arc<dyn SessionRepository>,
}

impl SessionCreator {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn execute(
        &self,
        user_id: String,
        session_id: String,
        login_at: OffsetDateTime,
        user_is_admin: bool,
    ) -> Result<(), String> {
        let new_session = UserSession::new(user_id, session_id, login_at, user_is_admin);

        self.session_repository
            .save(new_session)
            .await
            .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::session::domain::repository::tests::MockSessionRepository;

    #[tokio::test]
    async fn should_creates_a_new_session() {
        let mut session_repository = MockSessionRepository::new();
        session_repository
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let session_creator = SessionCreator::new(Arc::new(session_repository));

        let user_id = "user-id".to_string();
        let session_id = "session-id".to_string();
        let login_at = OffsetDateTime::now_utc();
        let user_is_admin = false;

        let result = session_creator
            .execute(user_id, session_id, login_at, user_is_admin)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_returns_an_error_when_fails_to_create_a_new_session() {
        let mut session_repository = MockSessionRepository::new();
        session_repository.expect_save().times(1).returning(|_| {
            Err(
                shared::domain::base_errors::BaseRepositoryError::UnexpectedError(
                    "Error".to_string(),
                ),
            )
        });

        let session_creator = SessionCreator::new(Arc::new(session_repository));

        let user_id = "user-id".to_string();
        let session_id = "session-id".to_string();
        let login_at = OffsetDateTime::now_utc();
        let user_is_admin = false;

        let result = session_creator
            .execute(user_id, session_id, login_at, user_is_admin)
            .await;

        assert!(result.is_err());
    }
}
