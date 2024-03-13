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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreatedAt(OffsetDateTime);

impl UserCreatedAt {
    pub fn new(value: String) -> Result<Self, String> {
        Ok(UserCreatedAt(
            OffsetDateTime::parse(&value, &Iso8601::DEFAULT).map_err(|e| e.to_string())?,
        ))
    }

    pub fn from_offset(offset: OffsetDateTime) -> Self {
        UserCreatedAt(offset)
    }

    pub fn value(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Display for UserCreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUpdatedAt(OffsetDateTime);

impl UserUpdatedAt {
    pub fn new(value: String) -> Result<Self, String> {
        Ok(UserUpdatedAt(
            OffsetDateTime::parse(&value, &Iso8601::DEFAULT).map_err(|e| e.to_string())?,
        ))
    }

    pub fn from_offset(offset: OffsetDateTime) -> Self {
        UserUpdatedAt(offset)
    }

    pub fn value(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Display for UserUpdatedAt {
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
    created_at: UserCreatedAt,
    updated_at: UserUpdatedAt,

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
        id: UserId,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> User {
        User {
            id,
            email,
            password,
            full_name,
            created_at,
            updated_at,

            events: vec![],
        }
    }

    pub fn new_user(
        id: UserId,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> Result<User, String> {
        let mut user = User::new(id, email, password, full_name, created_at, updated_at);

        user.record(Arc::new(UserCreatedEvent::new(
            user.id.to_string(),
            user.email.to_string(),
            user.password.to_string(),
            user.full_name.to_string(),
            user.created_at.to_string(),
            user.updated_at.to_string(),
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

    pub fn created_at(&self) -> &UserCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UserUpdatedAt {
        &self.updated_at
    }

    pub fn update(
        &mut self,
        password: Option<UserPassword>,
        full_name: Option<UserFullName>,
        updated_at: UserUpdatedAt,
    ) -> Result<(), String> {
        if let Some(password) = password {
            self.password = password;
        }

        if let Some(full_name) = full_name {
            self.full_name = full_name;
        }

        if self.updated_at.0.ge(&updated_at.0) {
            return Err("Updated at is older or equal than current updated at".to_string());
        }

        self.updated_at = updated_at;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use fake::{
        faker::{internet::en::SafeEmail, name::en::Name, time::en::DateTimeAfter},
        Fake, Faker,
    };
    use shared::common::domain::utils::MINIMUM_DATE_PERMITTED;

    pub struct UserIdMother;

    impl UserIdMother {
        pub fn create(value: String) -> UserId {
            UserId::new(value).unwrap()
        }

        pub fn random() -> UserId {
            UserId::new(Uuid::now_v7().to_string()).unwrap()
        }
    }

    pub struct UserEmailMother;

    impl UserEmailMother {
        pub fn create(value: String) -> UserEmail {
            UserEmail::new(value).unwrap()
        }

        pub fn random() -> UserEmail {
            UserEmail::new(SafeEmail().fake::<String>()).unwrap()
        }
    }

    pub struct UserPasswordMother;

    impl UserPasswordMother {
        pub fn create(value: String) -> UserPassword {
            UserPassword::new(value).unwrap()
        }

        pub fn random() -> UserPassword {
            UserPassword::new(Faker.fake::<String>()).unwrap()
        }
    }

    pub struct UserFullNameMother;

    impl UserFullNameMother {
        pub fn create(value: String) -> UserFullName {
            UserFullName::new(value).unwrap()
        }

        pub fn random() -> UserFullName {
            UserFullName::new(Name().fake::<String>()).unwrap()
        }
    }

    pub struct UserCreatedAtMother;

    impl UserCreatedAtMother {
        pub fn create(value: String) -> UserCreatedAt {
            UserCreatedAt::new(value).unwrap()
        }

        pub fn random() -> UserCreatedAt {
            UserCreatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake::<String>()).unwrap()
        }
    }

    pub struct UserUpdatedAtMother;

    impl UserUpdatedAtMother {
        pub fn create(value: String) -> UserUpdatedAt {
            UserUpdatedAt::new(value).unwrap()
        }

        fn random_after(date: OffsetDateTime) -> UserUpdatedAt {
            let updated_at: OffsetDateTime = DateTimeAfter(date).fake();
            UserUpdatedAt::new(updated_at.format(&Iso8601::DEFAULT).unwrap()).unwrap()
        }

        pub fn random_after_created(created_at: &UserCreatedAt) -> UserUpdatedAt {
            Self::random_after(created_at.0)
        }

        pub fn random_after_updated(updated_at: &UserUpdatedAt) -> UserUpdatedAt {
            Self::random_after(updated_at.0)
        }

        pub fn random() -> UserUpdatedAt {
            UserUpdatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake::<String>()).unwrap()
        }
    }

    pub struct UserMother;

    impl UserMother {
        pub fn create(
            id: UserId,
            email: UserEmail,
            password: UserPassword,
            full_name: UserFullName,
            created_at: UserCreatedAt,
            updated_at: UserUpdatedAt,
        ) -> User {
            User::new_user(id, email, password, full_name, created_at, updated_at).unwrap()
        }

        pub fn random() -> User {
            let created_at = UserCreatedAtMother::random();
            let updated_at = UserUpdatedAtMother::random_after_created(&created_at);
            User::new_user(
                UserId::new(Uuid::now_v7().to_string()).unwrap(),
                UserEmail::new(Faker.fake::<String>()).unwrap(),
                UserPassword::new(Faker.fake::<String>()).unwrap(),
                UserFullName::new(Faker.fake::<String>()).unwrap(),
                created_at,
                updated_at,
            )
            .unwrap()
        }
    }
}
