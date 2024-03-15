use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseRepositoryError {
    AlreadyExists,
    NotFound,
    ConnectionError,
    CriteriaCoverterError(String),
    UnexpectedError(String),
}

impl BaseRepositoryError {
    pub fn message(&self) -> String {
        match self {
            BaseRepositoryError::AlreadyExists => "Already exists".to_string(),
            BaseRepositoryError::NotFound => "Not found".to_string(),
            BaseRepositoryError::ConnectionError => "Connection error".to_string(),
            BaseRepositoryError::CriteriaCoverterError(message) => {
                format!("Criteria error: {}", message)
            }
            BaseRepositoryError::UnexpectedError(message) => {
                format!("Unexpected error: {}", message)
            }
        }
    }
}

impl Error for BaseRepositoryError {}
impl Display for BaseRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
