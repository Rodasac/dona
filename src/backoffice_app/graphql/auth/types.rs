use async_graphql::SimpleObject;
use backoffice::auth::application::response::UserResponse;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub profile_picture: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<UserResponse> for User {
    fn from(value: UserResponse) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            full_name: value.full_name,
            profile_picture: value.profile_picture,
            created_at: OffsetDateTime::parse(value.created_at.as_str(), &Rfc3339).unwrap(),
            updated_at: OffsetDateTime::parse(value.updated_at.as_str(), &Rfc3339).unwrap(),
        }
    }
}
