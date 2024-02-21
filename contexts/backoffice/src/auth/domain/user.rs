use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};
use shared::common::domain::bus::event::Event;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use uuid::Uuid;

use super::user_events::UserCreatedEvent;

pub const ERR_INVALID_USER_ID: &str = "Invalid user id";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: String) -> Result<Self, String> {
        let id = Uuid::parse_str(&value);
        match id {
            Ok(_) => Ok(UserId(value)),
            Err(_) => Err(ERR_INVALID_USER_ID.to_string()),
        }
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_EMAIL: &str = "Invalid email";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err(ERR_INVALID_EMAIL.to_string());
        }
        Ok(UserEmail(value))
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_PASSWORD: &str = "Invalid password";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err(ERR_INVALID_PASSWORD.to_string());
        }
        Ok(UserPassword(value))
    }
}

impl Display for UserPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_FULL_NAME: &str = "Invalid full name";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFullName(String);

impl UserFullName {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err(ERR_INVALID_FULL_NAME.to_string());
        }
        Ok(UserFullName(value))
    }
}

impl Display for UserFullName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct User {
    id: UserId,
    email: UserEmail,
    password: UserPassword,
    full_name: UserFullName,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,

    events: Vec<Arc<dyn Event>>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, email: {}, created_at: {}, updated_at: {} }}",
            self.id, self.email, self.created_at, self.updated_at
        )
    }
}

impl User {
    pub fn new(
        id: String,
        email: String,
        password: String,
        full_name: String,
        created_at: String,
        updated_at: String,
    ) -> Result<User, String> {
        let id = UserId::new(id)?;
        let email = UserEmail::new(email)?;
        let password = UserPassword::new(password)?;
        let full_name = UserFullName::new(full_name)?;
        let created_at =
            OffsetDateTime::parse(&created_at, &Iso8601::DEFAULT).map_err(|e| e.to_string())?;
        let updated_at =
            OffsetDateTime::parse(&updated_at, &Iso8601::DEFAULT).map_err(|e| e.to_string())?;

        let mut user = User {
            id,
            email,
            password,
            full_name,
            created_at,
            updated_at,

            events: vec![],
        };
        user.record(Arc::new(UserCreatedEvent::new(
            user.id.to_string(),
            user.email.to_string(),
            user.password.to_string(),
            user.full_name.to_string(),
            user.created_at,
            user.updated_at,
        )));

        Ok(user)
    }

    pub fn record(&mut self, event: Arc<dyn Event>) {
        self.events.push(event);
    }

    pub fn pull_events(&mut self) -> Vec<Arc<dyn Event>> {
        let events = self.events.clone();
        self.events = vec![];
        events
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }

    pub fn full_name(&self) -> &UserFullName {
        &self.full_name
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }
}
