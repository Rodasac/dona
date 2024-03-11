use lazy_static::lazy_static;
use time::{Date, OffsetDateTime, Time};

lazy_static! {
    pub static ref MINIMUM_DATE_PERMITTED: OffsetDateTime = OffsetDateTime::new_utc(
        Date::from_calendar_date(1, time::Month::January, 1).unwrap(),
        Time::MIDNIGHT,
    );
}
