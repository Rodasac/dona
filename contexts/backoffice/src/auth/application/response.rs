use serde::{Deserialize, Serialize};
use shared::domain::bus::query::Response;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub profile_picture: Option<String>,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Response for UserResponse {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
}

impl Response for UsersResponse {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
