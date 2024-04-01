use std::fmt::Display;
use std::sync::Arc;

use rust_decimal::Decimal;
use shared::domain::{bus::event::Event, utils::is_uuid, value_objects::user_id::UserId};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::shared::domain::dona::DonaOptionMethod;

use super::{
    dona_confirmed_event::DonaConfirmedEvent, dona_created_event::DonaCreatedEvent,
    dona_rejected_event::DonaRejectedEvent,
};

pub const ERR_INVALID_DONA_ID: &str = "Invalid Dona ID";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaId(String);

impl DonaId {
    pub fn new(value: String) -> Result<Self, String> {
        if is_uuid(&value) {
            Ok(Self(value))
        } else {
            Err(ERR_INVALID_DONA_ID.to_string())
        }
    }
}

impl Display for DonaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_DONA_MSG: &str = "Invalid Dona Message";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaMsg(String);

impl DonaMsg {
    pub fn new(value: String) -> Result<Self, String> {
        if !value.is_empty() {
            Ok(Self(value))
        } else {
            Err(ERR_INVALID_DONA_MSG.to_string())
        }
    }
}

impl Display for DonaMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_DONA_AMOUNT: &str = "Invalid Dona Amount";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaAmount(Decimal);

impl DonaAmount {
    pub fn new(value: Decimal) -> Result<Self, String> {
        if value > Decimal::ZERO {
            Ok(Self(value))
        } else {
            Err(ERR_INVALID_DONA_AMOUNT.to_string())
        }
    }
}

impl Display for DonaAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const ERR_INVALID_DONA_STATUS: &str = "Invalid Dona Status";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DonaStatus {
    Pending,
    Confirmed,
    Rejected,
}

impl DonaStatus {
    pub fn new(value: String) -> Result<Self, String> {
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "confirmed" => Ok(Self::Confirmed),
            "rejected" => Ok(Self::Rejected),
            _ => Err(ERR_INVALID_DONA_STATUS.to_string()),
        }
    }
}

impl Display for DonaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Rejected => write!(f, "rejected"),
        }
    }
}

pub const ERR_INVALID_DONA_CREATED_AT: &str = "Invalid Dona Created At";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaCreatedAt(OffsetDateTime);

impl DonaCreatedAt {
    pub fn new(value: OffsetDateTime) -> Result<Self, String> {
        if value < OffsetDateTime::now_utc() {
            Ok(Self(value))
        } else {
            Err(ERR_INVALID_DONA_CREATED_AT.to_string())
        }
    }
}

impl Display for DonaCreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

pub const ERR_INVALID_DONA_UPDATED_AT: &str = "Invalid Dona Updated At";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DonaUpdatedAt(OffsetDateTime);

impl DonaUpdatedAt {
    pub fn new(value: OffsetDateTime) -> Result<Self, String> {
        if value < OffsetDateTime::now_utc() {
            Ok(Self(value))
        } else {
            Err(ERR_INVALID_DONA_UPDATED_AT.to_string())
        }
    }
}

impl Display for DonaUpdatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&Rfc3339).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Dona {
    id: DonaId,
    msg: DonaMsg,
    amount: DonaAmount,
    status: DonaStatus,
    method: DonaOptionMethod,
    user_id: UserId,
    sender_id: UserId,
    created_at: DonaCreatedAt,
    updated_at: DonaUpdatedAt,

    events: Vec<Arc<dyn Event>>,
}

impl PartialEq for Dona {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.msg == other.msg
            && self.amount == other.amount
            && self.status == other.status
            && self.method == other.method
            && self.user_id == other.user_id
            && self.sender_id == other.sender_id
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
    }
}

impl Eq for Dona {}

impl Display for Dona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dona: [id: {}, msg: {}, amount: {}, status: {}, method: {}, created_at: {}, updated_at: {}]",
            self.id, self.msg, self.amount, self.status, self.method, self.created_at, self.updated_at
        )
    }
}

impl Dona {
    pub(crate) fn new(
        id: String,
        msg: String,
        amount: Decimal,
        status: String,
        method: String,
        user_id: String,
        sender_id: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        Ok(Self {
            id: DonaId::new(id)?,
            msg: DonaMsg::new(msg)?,
            amount: DonaAmount::new(amount)?,
            status: DonaStatus::new(status)?,
            method: DonaOptionMethod::new(method)?,
            user_id: UserId::new(user_id)?,
            sender_id: UserId::new(sender_id)?,
            created_at: DonaCreatedAt::new(created_at)?,
            updated_at: DonaUpdatedAt::new(updated_at)?,
            events: vec![],
        })
    }

    pub fn create(
        id: String,
        msg: String,
        amount: Decimal,
        status: String,
        method: String,
        user_id: String,
        sender_id: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, String> {
        let mut dona = Self::new(
            id.clone(),
            msg.clone(),
            amount,
            status,
            method,
            user_id.clone(),
            sender_id.clone(),
            created_at,
            updated_at,
        )?;

        let event = DonaCreatedEvent::new(
            id,
            msg,
            amount.to_string(),
            user_id,
            sender_id,
            created_at.to_string(),
            updated_at.to_string(),
        );
        dona.record(Arc::new(event));

        Ok(dona)
    }

    pub fn confirm(&mut self, ocurred_at: OffsetDateTime) {
        self.status = DonaStatus::Confirmed;
        self.updated_at = DonaUpdatedAt::new(ocurred_at).unwrap();

        let event = DonaConfirmedEvent::new(self.id.to_string(), self.updated_at.to_string());

        self.record(Arc::new(event));
    }

    pub fn reject(&mut self, ocurred_at: OffsetDateTime) {
        self.status = DonaStatus::Rejected;
        self.updated_at = DonaUpdatedAt::new(ocurred_at).unwrap();

        let event = DonaRejectedEvent::new(self.id.to_string(), self.updated_at.to_string());

        self.record(Arc::new(event));
    }

    pub fn record(&mut self, event: Arc<dyn Event>) {
        self.events.push(event);
    }

    pub fn pull_events(&mut self) -> Vec<Arc<dyn Event>> {
        let events = self.events.clone();
        self.events = vec![];
        events
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn msg(&self) -> String {
        self.msg.to_string()
    }

    pub fn amount(&self) -> Decimal {
        self.amount.0
    }

    pub fn status(&self) -> String {
        self.status.to_string()
    }

    pub fn is_confirmed(&self) -> bool {
        self.status == DonaStatus::Confirmed
    }

    pub fn method(&self) -> String {
        self.method.to_string()
    }

    pub fn user_id(&self) -> String {
        self.user_id.to_string()
    }

    pub fn sender_id(&self) -> String {
        self.sender_id.to_string()
    }

    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at.0
    }

    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at.0
    }
}

pub mod tests {
    use crate::shared::domain::dona::tests::DonaOptionMethodMother;

    use super::*;

    use fake::{
        decimal::PositiveDecimal,
        faker::{lorem::en::Sentence, time::en::DateTimeAfter},
        Dummy, Fake,
    };
    use rand::seq::SliceRandom;
    use shared::domain::{
        utils::{new_uuid, MINIMUM_DATE_PERMITTED},
        value_objects::user_id::tests::UserIdMother,
    };

    pub struct DonaIdMother;

    impl DonaIdMother {
        pub fn create(value: Option<String>) -> DonaId {
            match value {
                Some(value) => DonaId::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> DonaId {
            DonaId::new(new_uuid()).unwrap()
        }
    }

    pub struct DonaMsgMother;

    impl DonaMsgMother {
        pub fn create(value: Option<String>) -> DonaMsg {
            match value {
                Some(value) => DonaMsg::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> DonaMsg {
            DonaMsg::new(Sentence(1..10).fake()).unwrap()
        }
    }

    pub struct DonaAmountMother;

    impl DonaAmountMother {
        pub fn create(value: Option<Decimal>) -> DonaAmount {
            match value {
                Some(value) => DonaAmount::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> DonaAmount {
            DonaAmount::new(PositiveDecimal.fake()).unwrap()
        }
    }

    struct DonaStatusFaker;

    impl Dummy<DonaStatusFaker> for DonaStatus {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &DonaStatusFaker, rng: &mut R) -> Self {
            let values = vec![DonaStatus::Pending, DonaStatus::Confirmed];
            values.choose(rng).unwrap().clone()
        }
    }

    pub struct DonaStatusMother;

    impl DonaStatusMother {
        pub fn create(value: Option<String>) -> DonaStatus {
            match value {
                Some(value) => DonaStatus::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> DonaStatus {
            DonaStatusFaker.fake()
        }
    }

    pub struct DonaCreatedAtMother;

    impl DonaCreatedAtMother {
        pub fn create(value: Option<OffsetDateTime>) -> DonaCreatedAt {
            match value {
                Some(value) => DonaCreatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn random() -> DonaCreatedAt {
            DonaCreatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }
    }

    pub struct DonaUpdatedAtMother;

    impl DonaUpdatedAtMother {
        pub fn create(value: Option<OffsetDateTime>) -> DonaUpdatedAt {
            match value {
                Some(value) => DonaUpdatedAt::new(value).unwrap(),
                None => Self::random(),
            }
        }

        pub fn create_after(value: OffsetDateTime) -> DonaUpdatedAt {
            DonaUpdatedAt::new(DateTimeAfter(value).fake()).unwrap()
        }

        pub fn random() -> DonaUpdatedAt {
            DonaUpdatedAt::new(DateTimeAfter(*MINIMUM_DATE_PERMITTED).fake()).unwrap()
        }
    }

    pub struct DonaMother;

    impl DonaMother {
        pub fn create(
            id: Option<String>,
            msg: Option<String>,
            amount: Option<Decimal>,
            status: Option<String>,
            method: Option<String>,
            user_id: Option<String>,
            sender_id: Option<String>,
            created_at: Option<OffsetDateTime>,
            updated_at: Option<OffsetDateTime>,
        ) -> Dona {
            Dona {
                id: DonaIdMother::create(id),
                msg: DonaMsgMother::create(msg),
                amount: DonaAmountMother::create(amount),
                status: DonaStatusMother::create(status),
                method: DonaOptionMethodMother::create(method),
                user_id: UserIdMother::create(user_id),
                sender_id: UserIdMother::create(sender_id),
                created_at: DonaCreatedAtMother::create(created_at),
                updated_at: DonaUpdatedAtMother::create(updated_at),

                events: vec![],
            }
        }

        pub fn random() -> Dona {
            Self::create(None, None, None, None, None, None, None, None, None)
        }
    }
}
