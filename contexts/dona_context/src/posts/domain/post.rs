use std::fmt::Display;
use std::sync::Arc;

use shared::domain::{bus::event::Event, utils::is_uuid, value_objects::user_id::UserId};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::{
    post_content_updated_event::PostContentUpdatedEvent, post_created_event::PostCreatedEvent,
    post_is_nsfw_updated_event::PostIsNsfwUpdatedEvent,
    post_picture_updated_event::PostPictureUpdatedEvent,
};

pub const ERR_INVALID_POST_ID: &str = "Invalid post id";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostId(String);

impl PostId {
    pub fn new(id: String) -> Result<Self, String> {
        if is_uuid(&id) {
            Ok(Self(id))
        } else {
            Err(ERR_INVALID_POST_ID.to_string())
        }
    }
}

impl Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_POST_CONTENT: &str = "Invalid post content";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(content: String) -> Result<Self, String> {
        if content.is_empty() {
            Err(ERR_INVALID_POST_CONTENT.to_string())
        } else {
            Ok(Self(content))
        }
    }
}

impl Display for PostContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_POST_PICTURE: &str = "Invalid post picture";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostPicture(String);

impl PostPicture {
    pub fn new(picture: String) -> Result<Self, String> {
        if picture.is_empty() {
            Err(ERR_INVALID_POST_PICTURE.to_string())
        } else {
            Ok(Self(picture))
        }
    }
}

impl Display for PostPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostIsNSFW(bool);

impl PostIsNSFW {
    pub fn new(is_nsfw: bool) -> Self {
        Self(is_nsfw)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

impl Display for PostIsNSFW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_POST_CREATED_AT: &str = "Invalid post created at";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostCreatedAt(OffsetDateTime);

impl PostCreatedAt {
    pub fn new(created_at: OffsetDateTime) -> Result<Self, String> {
        if created_at < OffsetDateTime::now_utc() {
            Ok(Self(created_at))
        } else {
            Err(ERR_INVALID_POST_CREATED_AT.to_string())
        }
    }

    pub fn value(&self) -> OffsetDateTime {
        self.0
    }
}

impl Display for PostCreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

pub const ERR_INVALID_POST_UPDATED_AT: &str = "Invalid post updated at";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PostUpdatedAt(OffsetDateTime);

impl PostUpdatedAt {
    pub fn new(updated_at: OffsetDateTime) -> Result<Self, String> {
        if updated_at < OffsetDateTime::now_utc() {
            Ok(Self(updated_at))
        } else {
            Err(ERR_INVALID_POST_UPDATED_AT.to_string())
        }
    }

    pub fn value(&self) -> OffsetDateTime {
        self.0
    }
}

impl Display for PostUpdatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    id: PostId,
    user_id: UserId,
    content: PostContent,
    picture: Option<PostPicture>,
    is_nsfw: PostIsNSFW,
    created_at: PostCreatedAt,
    updated_at: PostUpdatedAt,

    events: Vec<Arc<dyn Event>>,
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.content == other.content
            && self.picture == other.picture
            && self.is_nsfw == other.is_nsfw
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
    }
}

impl Eq for Post {}

impl Post {
    pub(crate) fn new(
        id: String,
        user_id: String,
        content: String,
        picture: Option<String>,
        is_nsfw: bool,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        Ok(Self {
            id: PostId::new(id)?,
            user_id: UserId::new(user_id)?,
            content: PostContent::new(content)?,
            picture: picture.map(PostPicture::new).transpose()?,
            is_nsfw: PostIsNSFW::new(is_nsfw),
            created_at: PostCreatedAt::new(created_at)?,
            updated_at: PostUpdatedAt::new(updated_at)?,
            events: vec![],
        })
    }

    pub fn create(
        id: String,
        user_id: String,
        content: String,
        picture: Option<String>,
        is_nsfw: bool,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        let mut post = Self::new(
            id, user_id, content, picture, is_nsfw, created_at, updated_at,
        )?;
        let event = PostCreatedEvent::new(
            post.id(),
            post.user_id(),
            post.content(),
            post.picture(),
            post.is_nsfw(),
            post.created_at_str(),
            post.updated_at_str(),
        );

        post.events.push(Arc::new(event));

        Ok(post)
    }

    pub fn update_content(
        &mut self,
        content: String,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        self.content = PostContent::new(content)?;
        self.updated_at = PostUpdatedAt::new(updated_at)?;

        self.events.push(Arc::new(PostContentUpdatedEvent::new(
            self.id(),
            self.user_id(),
            self.content(),
            self.updated_at_str(),
        )));

        Ok(())
    }

    pub fn update_picture(
        &mut self,
        picture: Option<String>,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        self.picture = picture.map(PostPicture::new).transpose()?;
        self.updated_at = PostUpdatedAt::new(updated_at)?;

        self.events.push(Arc::new(PostPictureUpdatedEvent::new(
            self.id(),
            self.user_id(),
            self.picture(),
            self.updated_at_str(),
        )));

        Ok(())
    }

    pub fn update_is_nsfw(
        &mut self,
        is_nsfw: bool,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        self.is_nsfw = PostIsNSFW::new(is_nsfw);
        self.updated_at = PostUpdatedAt::new(updated_at)?;

        self.events.push(Arc::new(PostIsNsfwUpdatedEvent::new(
            self.id(),
            self.user_id(),
            self.is_nsfw(),
            self.updated_at_str(),
        )));

        Ok(())
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn user_id(&self) -> String {
        self.user_id.to_string()
    }

    pub fn content(&self) -> String {
        self.content.to_string()
    }

    pub fn picture(&self) -> Option<String> {
        self.picture.as_ref().map(|picture| picture.to_string())
    }

    pub fn is_nsfw(&self) -> bool {
        self.is_nsfw.value()
    }

    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at.value()
    }

    pub fn created_at_str(&self) -> String {
        self.created_at.to_string()
    }

    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at.value()
    }

    pub fn updated_at_str(&self) -> String {
        self.updated_at.to_string()
    }
}

pub mod tests {
    use super::*;

    use fake::{
        faker::{filesystem::en::FileName, lorem::en::Sentence, time::en::DateTimeAfter},
        Fake,
    };
    use shared::domain::{
        utils::{new_uuid, MINIMUM_DATE_PERMITTED},
        value_objects::user_id::tests::UserIdMother,
    };

    pub struct PostIdMother;

    impl PostIdMother {
        pub fn random() -> PostId {
            PostId::new(new_uuid()).unwrap()
        }

        pub fn create(value: Option<String>) -> PostId {
            match value {
                Some(value) => PostId::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct PostContentMother;

    impl PostContentMother {
        pub fn random() -> PostContent {
            PostContent::new(Sentence(1..10).fake()).unwrap()
        }

        pub fn create(value: Option<String>) -> PostContent {
            match value {
                Some(value) => PostContent::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct PostPictureMother;

    impl PostPictureMother {
        pub fn random() -> PostPicture {
            PostPicture::new(format!("{}.{}", FileName().fake::<String>(), "jpg")).unwrap()
        }

        pub fn create(value: Option<String>) -> PostPicture {
            match value {
                Some(value) => PostPicture::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct PostIsNSFWMother;

    impl PostIsNSFWMother {
        pub fn random() -> PostIsNSFW {
            PostIsNSFW::new(rand::random())
        }

        pub fn create(value: Option<bool>) -> PostIsNSFW {
            match value {
                Some(value) => PostIsNSFW::new(value),
                None => Self::random(),
            }
        }
    }

    pub struct PostCreatedAtMother;

    impl PostCreatedAtMother {
        pub fn random() -> PostCreatedAt {
            PostCreatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }

        pub fn create(value: Option<OffsetDateTime>) -> PostCreatedAt {
            match value {
                Some(value) => PostCreatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct PostUpdatedAtMother;

    impl PostUpdatedAtMother {
        pub fn random() -> PostUpdatedAt {
            PostUpdatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }

        pub fn create(value: Option<OffsetDateTime>) -> PostUpdatedAt {
            match value {
                Some(value) => PostUpdatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn create_after(value: OffsetDateTime) -> PostUpdatedAt {
            PostUpdatedAt::new(DateTimeAfter(value).fake()).unwrap()
        }
    }

    pub struct PostMother;

    impl PostMother {
        pub fn random() -> Post {
            Self::create(None, None, None, None, None, None, None)
        }

        pub fn create(
            id: Option<String>,
            user_id: Option<String>,
            content: Option<String>,
            picture: Option<Option<String>>,
            is_nsfw: Option<bool>,
            created_at: Option<OffsetDateTime>,
            updated_at: Option<OffsetDateTime>,
        ) -> Post {
            Post {
                id: PostIdMother::create(id),
                user_id: UserIdMother::create(user_id),
                content: PostContentMother::create(content),
                picture: picture
                    .map(|po| po.map(|p| PostPictureMother::create(Some(p))))
                    .unwrap_or(Some(PostPictureMother::random())),
                is_nsfw: PostIsNSFWMother::create(is_nsfw),
                created_at: PostCreatedAtMother::create(created_at),
                updated_at: PostUpdatedAtMother::create(updated_at),

                events: vec![],
            }
        }
    }
}
