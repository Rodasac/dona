use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

pub trait UserPasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, HashError>;
    fn verify(&self, password: &str, hash: &str) -> Result<(), HashError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashError {
    InvalidHash,
    InvalidPassword,
    InternalError(String),
}

impl HashError {
    pub fn message(&self) -> String {
        match self {
            HashError::InvalidHash => "Invalid hash".to_string(),
            HashError::InvalidPassword => "Invalid password".to_string(),
            HashError::InternalError(message) => format!("Internal error: {}", message),
        }
    }
}

impl Error for HashError {}
impl Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash error: {}", self.message())
    }
}

#[cfg(test)]
pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub UserPasswordHasher {}
        impl UserPasswordHasher for UserPasswordHasher {
            fn hash(&self, password: &str) -> Result<String, HashError>;
            fn verify(&self, password: &str, hash: &str) -> Result<(), HashError>;
        }
    }
}
