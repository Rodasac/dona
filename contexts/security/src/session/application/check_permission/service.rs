use crate::session::domain::repository::SessionRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct PermissionChecker {
    session_repository: Arc<dyn SessionRepository>,
}

impl PermissionChecker {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn execute(&self, session_id: String, user_id: Option<String>) -> Result<(), String> {
        let session = self
            .session_repository
            .get(&session_id)
            .await
            .map_err(|err| err.to_string())?;

        if session.user_is_admin() {
            return Ok(());
        }

        if let Some(user_id) = user_id {
            if session.user_id().to_string() == user_id {
                return Ok(());
            }
        }

        Err("Unauthorized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use crate::session::domain::{repository::tests::MockSessionRepository, user::UserSession};

    use super::*;
    #[tokio::test]
    async fn should_return_ok_if_session_is_admin() {
        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            true,
        );
        let mut session_repo = MockSessionRepository::new();
        session_repo
            .expect_get()
            .withf(|session_id| session_id == "session_id")
            .returning(move |_| Ok(session.clone()));

        let permission_checker = PermissionChecker::new(Arc::new(session_repo));
        let result = permission_checker
            .execute("session_id".to_string(), None)
            .await;

        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn should_return_ok_if_user_id_is_the_same_as_session_user_id() {
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

        let permission_checker = PermissionChecker::new(Arc::new(session_repo));
        let result = permission_checker
            .execute("session_id".to_string(), Some("user_id".to_string()))
            .await;

        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn should_return_unauthorized_if_user_id_is_different_from_session_user_id() {
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

        let permission_checker = PermissionChecker::new(Arc::new(session_repo));
        let result = permission_checker
            .execute("session_id".to_string(), Some("other_user_id".to_string()))
            .await;

        assert_eq!(result, Err("Unauthorized".to_string()));
    }
}
