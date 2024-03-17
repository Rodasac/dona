use std::fmt::Display;

use super::{cursor::Cursor, filter::Filter, order::Order};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Criteria {
    filters: Vec<Filter>,
    order: Option<Order>,
    cursor: Option<Cursor>,
}

impl Criteria {
    pub fn new(filters: Vec<Filter>, order: Option<Order>, cursor: Option<Cursor>) -> Self {
        Self {
            filters,
            order,
            cursor,
        }
    }

    pub fn filters(&self) -> &[Filter] {
        &self.filters
    }

    pub fn order(&self) -> Option<&Order> {
        self.order.as_ref()
    }

    pub fn cursor(&self) -> Option<&Cursor> {
        self.cursor.as_ref()
    }

    pub fn has_filters(&self) -> bool {
        !self.filters.is_empty()
    }
}

impl Display for Criteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Criteria {{ filters: [")?;
        for filter in &self.filters {
            write!(f, "{}, ", filter)?;
        }
        write!(
            f,
            "], order: {:?}, cursor: {:?} }}",
            self.order, self.cursor
        )
    }
}
