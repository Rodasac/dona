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
