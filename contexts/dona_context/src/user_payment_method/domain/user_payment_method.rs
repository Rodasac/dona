use std::fmt::Display;
use std::sync::Arc;

use shared::domain::{bus::event::Event, utils::is_uuid, value_objects::user_id::UserId};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::shared::domain::dona::DonaOptionMethod;

use super::{
    user_payment_method_created_event::UserPaymentMethodCreatedEvent,
    user_payment_method_update_instructions_event::UserPaymentMethodInstructionsUpdatedEvent,
};

pub const ERR_INVALID_USER_PAYMENT_METHOD_ID: &str = "Invalid user payment method id";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPaymentMethodId(String);

impl UserPaymentMethodId {
    pub fn new(value: String) -> Result<Self, String> {
        if is_uuid(&value) {
            return Ok(Self(value));
        }

        return Err(ERR_INVALID_USER_PAYMENT_METHOD_ID.to_string());
    }
}

impl Display for UserPaymentMethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_USER_PAYMENT_METHOD_INSTRUCTIONS: &str =
    "Invalid user payment method instructions";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPaymentMethodInstructions(String);

impl UserPaymentMethodInstructions {
    pub fn new(value: String) -> Result<Self, String> {
        if !value.is_empty() {
            return Ok(Self(value));
        }

        return Err(ERR_INVALID_USER_PAYMENT_METHOD_INSTRUCTIONS.to_string());
    }
}

impl Display for UserPaymentMethodInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_USER_PAYMENT_METHOD_CREATED_AT: &str =
    "Invalid user payment method created at";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserPaymentMethodCreatedAt(OffsetDateTime);

impl UserPaymentMethodCreatedAt {
    pub fn new(value: OffsetDateTime) -> Result<Self, String> {
        if value < OffsetDateTime::now_utc() {
            return Ok(Self(value));
        }

        return Err(ERR_INVALID_USER_PAYMENT_METHOD_CREATED_AT.to_string());
    }

    pub fn value(&self) -> OffsetDateTime {
        self.0
    }
}

impl Display for UserPaymentMethodCreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

pub const ERR_INVALID_USER_PAYMENT_METHOD_UPDATED_AT: &str =
    "Invalid user payment method updated at";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserPaymentMethodUpdatedAt(OffsetDateTime);

impl UserPaymentMethodUpdatedAt {
    pub fn new(value: OffsetDateTime) -> Result<Self, String> {
        if value < OffsetDateTime::now_utc() {
            return Ok(Self(value));
        }

        return Err(ERR_INVALID_USER_PAYMENT_METHOD_UPDATED_AT.to_string());
    }

    pub fn value(&self) -> OffsetDateTime {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct UserPaymentMethod {
    id: UserPaymentMethodId,
    user_id: UserId,
    payment_method: DonaOptionMethod,
    instructions: UserPaymentMethodInstructions,
    created_at: UserPaymentMethodCreatedAt,
    updated_at: UserPaymentMethodUpdatedAt,

    events: Vec<Arc<dyn Event>>,
}

impl PartialEq for UserPaymentMethod {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.payment_method == other.payment_method
            && self.instructions == other.instructions
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
    }
}

impl Eq for UserPaymentMethod {}

impl UserPaymentMethod {
    pub(crate) fn new(
        id: String,
        user_id: String,
        payment_method: String,
        instructions: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        Ok(Self {
            id: UserPaymentMethodId::new(id)?,
            user_id: UserId::new(user_id)?,
            payment_method: DonaOptionMethod::new(payment_method)?,
            instructions: UserPaymentMethodInstructions::new(instructions)?,
            created_at: UserPaymentMethodCreatedAt::new(created_at)?,
            updated_at: UserPaymentMethodUpdatedAt::new(updated_at)?,

            events: vec![],
        })
    }

    pub fn create(
        id: String,
        user_id: String,
        payment_method: String,
        instructions: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        let mut method = Self::new(
            id,
            user_id,
            payment_method,
            instructions,
            created_at,
            updated_at,
        )?;

        method
            .events
            .push(Arc::new(UserPaymentMethodCreatedEvent::new(
                method.id().to_string(),
                method.user_id().to_string(),
                method.payment_method().to_string(),
                method.instructions().to_string(),
                method.created_at().to_string(),
                method.updated_at().to_string(),
            )));

        Ok(method)
    }

    pub fn update_instructions(
        &mut self,
        instructions: String,
        updated_at: OffsetDateTime,
    ) -> Result<(), String> {
        self.instructions = UserPaymentMethodInstructions::new(instructions)?;
        self.updated_at = UserPaymentMethodUpdatedAt::new(updated_at)?;

        self.events
            .push(Arc::new(UserPaymentMethodInstructionsUpdatedEvent::new(
                self.id().to_string(),
                self.user_id().to_string(),
                self.payment_method().to_string(),
                self.instructions().to_string(),
                self.created_at().to_string(),
                self.updated_at().to_string(),
            )));

        Ok(())
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn user_id(&self) -> String {
        self.user_id.to_string()
    }

    pub fn payment_method(&self) -> String {
        self.payment_method.to_string()
    }

    pub fn instructions(&self) -> String {
        self.instructions.to_string()
    }

    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at.value()
    }

    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at.value()
    }
}

pub mod tests {
    use crate::shared::domain::dona::tests::DonaOptionMethodMother;

    use super::*;

    use fake::{
        faker::{lorem::en::Sentence, time::en::DateTimeAfter},
        Fake,
    };
    use shared::domain::{
        utils::{new_uuid, MINIMUM_DATE_PERMITTED},
        value_objects::user_id::tests::UserIdMother,
    };

    pub struct UserPaymentMethodIdMother;

    impl UserPaymentMethodIdMother {
        pub fn random() -> UserPaymentMethodId {
            UserPaymentMethodId::new(new_uuid()).unwrap()
        }

        pub fn create(value: Option<String>) -> UserPaymentMethodId {
            match value {
                Some(value) => UserPaymentMethodId::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct UserPaymentMethodInstructionsMother;

    impl UserPaymentMethodInstructionsMother {
        pub fn random() -> UserPaymentMethodInstructions {
            UserPaymentMethodInstructions::new(Sentence(1..10).fake()).unwrap()
        }

        pub fn create(value: Option<String>) -> UserPaymentMethodInstructions {
            match value {
                Some(value) => UserPaymentMethodInstructions::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct UserPaymentMethodCreatedAtMother;

    impl UserPaymentMethodCreatedAtMother {
        pub fn random() -> UserPaymentMethodCreatedAt {
            UserPaymentMethodCreatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }

        pub fn create(value: Option<OffsetDateTime>) -> UserPaymentMethodCreatedAt {
            match value {
                Some(value) => UserPaymentMethodCreatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }

    pub struct UserPaymentMethodUpdatedAtMother;

    impl UserPaymentMethodUpdatedAtMother {
        pub fn random() -> UserPaymentMethodUpdatedAt {
            UserPaymentMethodUpdatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }

        pub fn create(value: Option<OffsetDateTime>) -> UserPaymentMethodUpdatedAt {
            match value {
                Some(value) => UserPaymentMethodUpdatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn create_after(value: OffsetDateTime) -> UserPaymentMethodUpdatedAt {
            UserPaymentMethodUpdatedAt::new(DateTimeAfter(value).fake()).unwrap()
        }
    }

    pub struct UserPaymentMethodMother;

    impl UserPaymentMethodMother {
        pub fn random() -> UserPaymentMethod {
            Self::create(None, None, None, None, None, None)
        }

        pub fn create(
            id: Option<String>,
            user_id: Option<String>,
            payment_method: Option<String>,
            instructions: Option<String>,
            created_at: Option<OffsetDateTime>,
            updated_at: Option<OffsetDateTime>,
        ) -> UserPaymentMethod {
            UserPaymentMethod::new(
                id.unwrap_or_else(|| new_uuid()),
                user_id.unwrap_or_else(|| UserIdMother::random().to_string()),
                payment_method.unwrap_or_else(|| DonaOptionMethodMother::random().to_string()),
                instructions.unwrap_or_else(|| Sentence(1..10).fake()),
                created_at.unwrap_or_else(|| DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()),
                updated_at.unwrap_or_else(|| DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()),
            )
            .unwrap()
        }
    }
}
