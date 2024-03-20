use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};
use shared::domain::bus::event::Event;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
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

pub const ERR_INVALID_USERNAME: &str = "Invalid username";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUsername(String);

impl UserUsername {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err(ERR_INVALID_USERNAME.to_string());
        }
        Ok(UserUsername(value))
    }
}

impl Display for UserUsername {
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
pub struct UserLastLogin(Option<OffsetDateTime>);

impl UserLastLogin {
    pub fn new(value: Option<OffsetDateTime>) -> Self {
        UserLastLogin(value)
    }

    pub fn value(&self) -> Option<&OffsetDateTime> {
        self.0.as_ref()
    }
}

impl TryFrom<String> for UserLastLogin {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "None" => Ok(UserLastLogin(None)),
            _ => Ok(UserLastLogin(Some(
                OffsetDateTime::parse(&value, &Rfc3339).map_err(|e| e.to_string())?,
            ))),
        }
    }
}

impl Display for UserLastLogin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value.format(&Rfc3339).unwrap()),
            None => write!(f, "NULL"),
        }
    }
}

pub const ERR_INVALID_PROFILE_PICTURE: &str = "Invalid profile picture";

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfilePicture(Option<String>);

impl UserProfilePicture {
    pub fn new(value: Option<String>) -> Result<Self, String> {
        if let Some(value) = &value {
            if value.is_empty() {
                return Err(ERR_INVALID_PROFILE_PICTURE.to_string());
            }
        }
        Ok(UserProfilePicture(value))
    }

    pub fn value(&self) -> Option<&String> {
        self.0.as_ref()
    }
}

impl Display for UserProfilePicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "NULL"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIsAdmin(bool);

impl UserIsAdmin {
    pub fn new(value: bool) -> Self {
        UserIsAdmin(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

impl Display for UserIsAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "true" } else { "false" })
    }
}

impl TryFrom<String> for UserIsAdmin {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "true" => Ok(UserIsAdmin(true)),
            "false" => Ok(UserIsAdmin(false)),
            _ => Err("Invalid value for UserIsAdmin".to_string()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreatedAt(OffsetDateTime);

impl UserCreatedAt {
    pub fn new(value: String) -> Result<Self, String> {
        Ok(UserCreatedAt(
            OffsetDateTime::parse(&value, &Rfc3339).map_err(|e| e.to_string())?,
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
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUpdatedAt(OffsetDateTime);

impl UserUpdatedAt {
    pub fn new(value: String) -> Result<Self, String> {
        Ok(UserUpdatedAt(
            OffsetDateTime::parse(&value, &Rfc3339).map_err(|e| e.to_string())?,
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
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    username: UserUsername,
    email: UserEmail,
    password: UserPassword,
    full_name: UserFullName,
    last_login: UserLastLogin,
    profile_picture: UserProfilePicture,
    is_admin: UserIsAdmin,
    created_at: UserCreatedAt,
    updated_at: UserUpdatedAt,

    events: Vec<Arc<dyn Event>>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.email == other.email
            && self.password == other.password
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
    }
}
impl Eq for User {}

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
        username: UserUsername,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        last_login: UserLastLogin,
        profile_picture: UserProfilePicture,
        is_admin: UserIsAdmin,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> User {
        User {
            id,
            username,
            email,
            password,
            full_name,
            last_login,
            profile_picture,
            is_admin,
            created_at,
            updated_at,

            events: vec![],
        }
    }

    pub fn new_user(
        id: UserId,
        username: UserUsername,
        email: UserEmail,
        password: UserPassword,
        full_name: UserFullName,
        profile_picture: UserProfilePicture,
        is_admin: UserIsAdmin,
        created_at: UserCreatedAt,
        updated_at: UserUpdatedAt,
    ) -> Result<User, String> {
        let mut user = User::new(
            id,
            username,
            email,
            password,
            full_name,
            UserLastLogin(None),
            profile_picture,
            is_admin,
            created_at,
            updated_at,
        );

        user.record(Arc::new(UserCreatedEvent::new(
            user.id.to_string(),
            user.username.to_string(),
            user.email.to_string(),
            user.password.to_string(),
            user.full_name.to_string(),
            user.is_admin.value(),
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

    pub fn username(&self) -> &UserUsername {
        &self.username
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

    pub fn last_login(&self) -> &UserLastLogin {
        &self.last_login
    }

    pub fn profile_picture(&self) -> &UserProfilePicture {
        &self.profile_picture
    }

    pub fn is_admin(&self) -> &UserIsAdmin {
        &self.is_admin
    }

    pub fn created_at(&self) -> &UserCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UserUpdatedAt {
        &self.updated_at
    }

    pub fn update(
        &mut self,
        username: Option<UserUsername>,
        password: Option<UserPassword>,
        full_name: Option<UserFullName>,
        profile_picture: Option<UserProfilePicture>,
        is_admin: Option<UserIsAdmin>,
        updated_at: UserUpdatedAt,
    ) -> Result<(), String> {
        if let Some(username) = username {
            self.username = username;
        }

        if let Some(password) = password {
            self.password = password;
        }

        if let Some(full_name) = full_name {
            self.full_name = full_name;
        }

        if let Some(profile_picture) = profile_picture {
            self.profile_picture = profile_picture;
        }

        if let Some(is_admin) = is_admin {
            self.is_admin = is_admin;
        }

        if self.updated_at.0.ge(&updated_at.0) {
            return Err("Updated at is older or equal than current updated at".to_string());
        }

        self.updated_at = updated_at;

        Ok(())
    }
}

pub mod tests {
    use std::ops::Range;

    use super::*;

    use fake::{
        faker::{
            boolean::en::Boolean,
            filesystem::en::FileName,
            internet::en::{Password, SafeEmail},
            name::en::FirstName,
            time::en::DateTimeAfter,
        },
        Fake,
    };
    use shared::domain::utils::MINIMUM_DATE_PERMITTED;

    pub struct UserIdMother;

    impl UserIdMother {
        pub fn create(value: String) -> UserId {
            UserId::new(value).unwrap()
        }

        pub fn random() -> UserId {
            UserId::new(Uuid::now_v7().to_string()).unwrap()
        }
    }

    pub struct UserUsernameMother;

    impl UserUsernameMother {
        pub fn create(value: String) -> UserUsername {
            UserUsername::new(value).unwrap()
        }

        pub fn random() -> UserUsername {
            UserUsername::new(SafeEmail().fake::<String>()).unwrap()
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
            UserPassword::new(Password(Range { start: 8, end: 32 }).fake()).unwrap()
        }
    }

    pub struct UserFullNameMother;

    impl UserFullNameMother {
        pub fn create(value: String) -> UserFullName {
            UserFullName::new(value).unwrap()
        }

        pub fn random() -> UserFullName {
            UserFullName::new(FirstName().fake::<String>()).unwrap()
        }
    }

    pub struct UserLastLoginMother;

    impl UserLastLoginMother {
        pub fn create(value: Option<OffsetDateTime>) -> UserLastLogin {
            UserLastLogin::new(value)
        }

        pub fn random() -> UserLastLogin {
            UserLastLogin::new(Some(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()))
        }
    }

    pub struct UserProfilePictureMother;

    impl UserProfilePictureMother {
        pub fn create(value: Option<String>) -> UserProfilePicture {
            UserProfilePicture::new(value).unwrap()
        }

        pub fn random() -> UserProfilePicture {
            UserProfilePicture::new(Some(format!("{}.{}", FileName().fake::<String>(), "jpg")))
                .unwrap()
        }
    }

    pub struct UserIsAdminMother;

    impl UserIsAdminMother {
        pub fn create(value: bool) -> UserIsAdmin {
            UserIsAdmin::new(value)
        }

        pub fn inverted(user_is_admin: &UserIsAdmin) -> UserIsAdmin {
            UserIsAdmin::new(!user_is_admin.0)
        }

        pub fn random() -> UserIsAdmin {
            UserIsAdmin::new(Boolean(50).fake())
        }
    }

    pub struct UserCreatedAtMother;

    impl UserCreatedAtMother {
        pub fn create(value: String) -> UserCreatedAt {
            UserCreatedAt::new(value).unwrap()
        }

        pub fn random() -> UserCreatedAt {
            let created_at: OffsetDateTime = DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake();
            UserCreatedAt::new(created_at.format(&Rfc3339).unwrap()).unwrap()
        }
    }

    pub struct UserUpdatedAtMother;

    impl UserUpdatedAtMother {
        pub fn create(value: String) -> UserUpdatedAt {
            UserUpdatedAt::new(value).unwrap()
        }

        fn random_after(date: OffsetDateTime) -> UserUpdatedAt {
            let updated_at: OffsetDateTime = DateTimeAfter(date).fake();
            UserUpdatedAt::new(updated_at.format(&Rfc3339).unwrap()).unwrap()
        }

        pub fn random_after_created(created_at: &UserCreatedAt) -> UserUpdatedAt {
            Self::random_after(created_at.0)
        }

        pub fn random_after_updated(updated_at: &UserUpdatedAt) -> UserUpdatedAt {
            Self::random_after(updated_at.0)
        }

        pub fn random() -> UserUpdatedAt {
            let updated_at: OffsetDateTime = DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake();
            UserUpdatedAt::new(updated_at.format(&Rfc3339).unwrap()).unwrap()
        }
    }

    pub struct UserMother;

    impl UserMother {
        pub fn create(
            id: UserId,
            username: UserUsername,
            email: UserEmail,
            password: UserPassword,
            full_name: UserFullName,
            profile_picture: UserProfilePicture,
            is_admin: UserIsAdmin,
            created_at: UserCreatedAt,
            updated_at: UserUpdatedAt,
        ) -> User {
            User::new_user(
                id,
                username,
                email,
                password,
                full_name,
                profile_picture,
                is_admin,
                created_at,
                updated_at,
            )
            .unwrap()
        }

        pub fn random() -> User {
            let created_at = UserCreatedAtMother::random();
            let updated_at = UserUpdatedAtMother::random_after_created(&created_at);
            User::new_user(
                UserIdMother::random(),
                UserUsernameMother::random(),
                UserEmailMother::random(),
                UserPasswordMother::random(),
                UserFullNameMother::random(),
                UserProfilePictureMother::random(),
                UserIsAdminMother::random(),
                created_at,
                updated_at,
            )
            .unwrap()
        }
    }
}
