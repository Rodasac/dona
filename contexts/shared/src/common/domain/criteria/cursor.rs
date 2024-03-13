use std::{collections::HashMap, fmt::Display};

use time::{format_description::well_known::Iso8601, OffsetDateTime};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AfterCursor {
    value: OffsetDateTime,
}

impl TryFrom<String> for AfterCursor {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("AfterCursor is empty".to_string());
        }

        let date = OffsetDateTime::parse(value.as_str(), &Iso8601::DEFAULT);
        if !date.is_ok() {
            return Err("AfterCursor is not a valid date".to_string());
        }

        Ok(Self {
            value: date.unwrap(),
        })
    }
}

impl Display for AfterCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format(&Iso8601::DEFAULT).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BeforeCursor {
    value: OffsetDateTime,
}

impl TryFrom<String> for BeforeCursor {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("BeforeCursor is empty".to_string());
        }

        let date = OffsetDateTime::parse(value.as_str(), &Iso8601::DEFAULT);
        if !date.is_ok() {
            return Err("BeforeCursor is not a valid date".to_string());
        }

        Ok(Self {
            value: date.unwrap(),
        })
    }
}

impl Display for BeforeCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format(&Iso8601::DEFAULT).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FirstField {
    value: usize,
}

impl FirstField {
    pub fn new(value: usize) -> Result<Self, String> {
        if value < 1 {
            return Err("FirstField is 0".to_string());
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl TryFrom<String> for FirstField {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value
            .parse::<usize>()
            .map_err(|_| "FirstField is not a valid number")?;

        Self::new(value)
    }
}

impl Display for FirstField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LastField {
    value: usize,
}

impl LastField {
    pub fn new(value: usize) -> Result<Self, String> {
        if value < 1 {
            return Err("LastField is 0".to_string());
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl TryFrom<String> for LastField {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value
            .parse::<usize>()
            .map_err(|_| "LastField is not a valid number")?;

        Self::new(value)
    }
}

impl Display for LastField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    after: Option<AfterCursor>,
    before: Option<BeforeCursor>,
    first: Option<FirstField>,
    last: Option<LastField>,
}

impl Cursor {
    pub fn new(
        after: Option<AfterCursor>,
        before: Option<BeforeCursor>,
        first: Option<FirstField>,
        last: Option<LastField>,
    ) -> Self {
        Self {
            after,
            before,
            first,
            last,
        }
    }

    pub fn new_validated(
        after: Option<AfterCursor>,
        before: Option<BeforeCursor>,
        first: Option<FirstField>,
        last: Option<LastField>,
    ) -> Result<Self, String> {
        if let Some(after) = &after {
            if let Some(before) = &before {
                if after.value > before.value {
                    return Err("AfterCursor is greater than BeforeCursor".to_string());
                }
            }
        }

        if first.is_some() && last.is_some() {
            return Err("first and last are both present".to_string());
        }

        Ok(Self {
            after,
            before,
            first,
            last,
        })
    }

    pub fn after(&self) -> Option<&AfterCursor> {
        self.after.as_ref()
    }

    pub fn before(&self) -> Option<&BeforeCursor> {
        self.before.as_ref()
    }

    pub fn first(&self) -> Option<&FirstField> {
        self.first.as_ref()
    }

    pub fn last(&self) -> Option<&LastField> {
        self.last.as_ref()
    }
}

impl Cursor {
    pub fn from_values(values: HashMap<String, String>) -> Result<Self, String> {
        let after = values
            .get("after")
            .map(|value| AfterCursor::try_from(value.to_string()))
            .transpose()?;

        let before = values
            .get("before")
            .map(|value| BeforeCursor::try_from(value.to_string()))
            .transpose()?;

        let first = values
            .get("first")
            .map(|value| FirstField::try_from(value.to_string()))
            .transpose()?;

        let last = values
            .get("last")
            .map(|value| LastField::try_from(value.to_string()))
            .transpose()?;

        Ok(Self {
            after,
            before,
            first,
            last,
        })
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.after
                .clone()
                .map(|after| after.to_string())
                .unwrap_or("none".to_string()),
            self.before
                .clone()
                .map(|before| before.to_string())
                .unwrap_or("none".to_string()),
            self.first
                .clone()
                .map(|first| first.to_string())
                .unwrap_or("none".to_string()),
            self.last
                .clone()
                .map(|last| last.to_string())
                .unwrap_or("none".to_string())
        )
    }
}
