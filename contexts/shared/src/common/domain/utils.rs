use lazy_static::lazy_static;
use time::{Date, OffsetDateTime, Time};

lazy_static! {
    pub static ref MINIMUM_DATE_PERMITTED: OffsetDateTime = OffsetDateTime::new_utc(
        Date::from_calendar_date(1, time::Month::January, 1).unwrap(),
        Time::MIDNIGHT,
    );
}

pub fn sanitize_string(input: &str) -> String {
    input
        .replace(
            &[
                '\n', '\t', '\r', '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{200F}',
                '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}', '\u{202F}', '\u{205F}',
                '\u{3000}', '\u{FEFF}', '\u{FFFC}', '\u{FFFD}', ';', ':', '"', '\'',
            ],
            "",
        )
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_string() {
        let sanitized = sanitize_string(
            "  \n\t\r\u{200B}\u{200C}\u{200D}\u{200E}\u{200F}\u{202A}\u{202B}\u{202C}\u{202D}\u{202E}\u{202F}\u{205F}\u{3000}\u{FEFF}\u{FFFC}\u{FFFD};:\"'  "
        );
        assert_eq!(sanitized, "    ");
    }
}
