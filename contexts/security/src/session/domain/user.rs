use serde::{Deserialize, Serialize};
use shared::domain::bus::query::Response;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserSession {
    user_id: String,
    session_id: String,
    login_at: OffsetDateTime,
    user_is_admin: bool,
}

impl UserSession {
    pub fn new(
        user_id: String,
        session_id: String,
        login_at: OffsetDateTime,
        user_is_admin: bool,
    ) -> Self {
        Self {
            user_id,
            session_id,
            login_at,
            user_is_admin,
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn login_at(&self) -> &OffsetDateTime {
        &self.login_at
    }

    pub fn user_is_admin(&self) -> bool {
        self.user_is_admin
    }
}

impl Response for UserSession {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
