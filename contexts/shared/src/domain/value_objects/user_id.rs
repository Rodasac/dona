use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::domain::utils::is_uuid;

pub const ERR_INVALID_USER_ID: &str = "Invalid user id";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: String) -> Result<Self, String> {
        if is_uuid(&value) {
            Ok(UserId(value))
        } else {
            Err(ERR_INVALID_USER_ID.to_string())
        }
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod tests {
    use crate::domain::utils::new_uuid;

    use super::*;

    pub struct UserIdMother;

    impl UserIdMother {
        pub fn create(value: Option<String>) -> UserId {
            match value {
                Some(value) => UserId::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> UserId {
            UserId::new(new_uuid()).unwrap()
        }
    }
}
