use async_graphql::{Enum, InputObject};
use time::OffsetDateTime;

use crate::domain::criteria::{
    cursor::{AfterCursor, BeforeCursor, Cursor, FirstField, LastField},
    filter::{Filter, FilterField, FilterOperator, FilterValue},
    order::{Order, OrderField, OrderType},
    Criteria,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, InputObject)]
#[graphql(name = "Criteria")]
pub struct CriteriaGql {
    pub filters: Vec<FilterGql>,
    pub order: Option<OrderGql>,
    pub cursor: Option<CursorGql>,
}

impl TryInto<Criteria> for CriteriaGql {
    type Error = String;

    fn try_into(self) -> Result<Criteria, Self::Error> {
        let filters = self
            .filters
            .into_iter()
            .map(|filter| filter.try_into())
            .collect::<Result<Vec<Filter>, String>>()?;
        let order = self.order.map(|order| order.try_into()).transpose()?;
        let cursor = self.cursor.map(|cursor| cursor.try_into()).transpose()?;
        Ok(Criteria::new(filters, order, cursor))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InputObject)]
#[graphql(name = "Filter")]
pub struct FilterGql {
    #[graphql(validator(chars_min_length = 1))]
    pub field: String,
    pub operator: FilterOperatorGql,
    #[graphql(validator(chars_min_length = 1))]
    pub value: String,
}

impl TryInto<Filter> for FilterGql {
    type Error = String;

    fn try_into(self) -> Result<Filter, Self::Error> {
        let field = FilterField::try_from(self.field)?;
        let operator = self.operator.into();
        let value = FilterValue::try_from(self.value)?;
        Ok(Filter::new(field, operator, value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
#[graphql(remote = "FilterOperator", name = "FilterOperator")]
pub enum FilterOperatorGql {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InputObject)]
#[graphql(name = "Order")]
pub struct OrderGql {
    #[graphql(validator(chars_min_length = 1))]
    pub order_by: String,
    pub order_type: OrderTypeGql,
}

impl TryInto<Order> for OrderGql {
    type Error = String;

    fn try_into(self) -> Result<Order, Self::Error> {
        let order_by = OrderField::try_from(self.order_by)?;
        Ok(Order::new(order_by, self.order_type.into()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
#[graphql(remote = "OrderType", name = "OrderType")]
pub enum OrderTypeGql {
    Asc,
    Desc,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, InputObject)]
#[graphql(name = "Cursor")]
pub struct CursorGql {
    pub after: Option<OffsetDateTime>,
    pub before: Option<OffsetDateTime>,
    pub first: Option<usize>,
    pub last: Option<usize>,
}

impl TryInto<Cursor> for CursorGql {
    type Error = String;

    fn try_into(self) -> Result<Cursor, Self::Error> {
        Cursor::new_validated(
            self.after.map(AfterCursor::new),
            self.before.map(BeforeCursor::new),
            self.first.map(FirstField::new).transpose()?,
            self.last.map(LastField::new).transpose()?,
        )
    }
}
