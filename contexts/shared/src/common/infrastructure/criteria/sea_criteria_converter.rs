use sea_orm::{
    prelude::Decimal, sea_query::SimpleExpr, ColumnFromStrErr, ColumnTrait, Condition,
    Cursor as SeaCursor, EntityTrait, FromQueryResult, QueryFilter, QueryOrder, Select,
    SelectModel, Value,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::common::domain::criteria::{
    cursor::Cursor,
    filter::{Filter, FilterOperator},
    order::{Order, OrderType},
    Criteria,
};

pub const DEFAULT_LIMIT: usize = 10;

pub fn sea_convert_criteria<'a, Columns, E>(
    query: &mut Select<E>,
    criteria: Criteria,
) -> Result<Select<E>, String>
where
    Columns: ColumnTrait<Err = ColumnFromStrErr>,
    E: EntityTrait,
    E::Model: Sync,
{
    let query_filters = query
        .clone()
        .filter(convert_filters::<Columns>(criteria.filters())?);

    let query_order = match criteria.order() {
        Some(order) => convert_order_to_column::<Columns, E>(order.to_owned(), query_filters)?,
        None => query_filters.clone(),
    };

    Ok(query_order)
}

fn convert_filters<Columns: ColumnTrait<Err = ColumnFromStrErr>>(
    filters: &[Filter],
) -> Result<Condition, String> {
    let mut conditions = Condition::all();

    for filter in filters {
        let column_conditions = convert_filter_to_column_conditions::<Columns>(filter.to_owned())?;
        conditions = conditions.add(column_conditions);
    }

    Ok(conditions)
}

fn parse_value(value: String) -> Value {
    if let Ok(v) = value.parse::<i32>() {
        Value::Int(Some(v))
    } else if let Ok(v) = value.parse::<bool>() {
        Value::Bool(Some(v))
    } else if let Ok(v) = value.parse::<Uuid>() {
        Value::Uuid(Some(Box::new(v)))
    } else if let Ok(v) = OffsetDateTime::parse(&value, &Rfc3339) {
        Value::TimeDateTimeWithTimeZone(Some(Box::new(v)))
    } else if let Ok(v) = value.parse::<Decimal>() {
        Value::Decimal(Some(Box::new(v)))
    } else {
        Value::String(Some(Box::new(value)))
    }
}

fn parse_values(values: Vec<String>) -> Vec<Value> {
    values.into_iter().map(parse_value).collect()
}

fn convert_filter_to_column_conditions<Columns: ColumnTrait<Err = ColumnFromStrErr>>(
    filter: Filter,
) -> Result<SimpleExpr, String> {
    let column =
        Columns::from_str(filter.field().to_string().as_str()).map_err(|e| e.to_string())?;
    let value = filter.value().to_string();

    let condition = match filter.operator() {
        FilterOperator::Equal => column.eq(parse_value(value)),
        FilterOperator::NotEqual => column.ne(parse_value(value)),
        FilterOperator::GreaterThan => column.gt(parse_value(value)),
        FilterOperator::GreaterThanOrEqual => column.gte(parse_value(value)),
        FilterOperator::LessThan => column.lt(parse_value(value)),
        FilterOperator::LessThanOrEqual => column.lte(parse_value(value)),
        FilterOperator::Like => column.like(value.to_string()),
        FilterOperator::NotLike => column.not_like(value.to_string()),
        FilterOperator::In => column.is_in(parse_values(
            value.split(',').map(|s| s.to_string()).collect(),
        )),
        FilterOperator::NotIn => column.is_not_in(parse_values(
            value.split(',').map(|s| s.to_string()).collect(),
        )),
    };

    Ok(condition)
}

fn convert_order_to_column<Columns: ColumnTrait<Err = ColumnFromStrErr>, E: EntityTrait>(
    order: Order,
    query: Select<E>,
) -> Result<Select<E>, String> {
    match order.order_type() {
        OrderType::Asc => Ok(query
            .order_by_asc(
                Columns::from_str(order.order_by().to_string().as_str())
                    .map_err(|e| e.to_string())?,
            )
            .clone()),
        OrderType::Desc => Ok(query
            .order_by_desc(
                Columns::from_str(order.order_by().to_string().as_str())
                    .map_err(|e| e.to_string())?,
            )
            .clone()),
        OrderType::None => Ok(query.clone()),
    }
}

pub fn convert_criteria_cursor<
    'a,
    Columns: ColumnTrait<Err = ColumnFromStrErr>,
    E: FromQueryResult,
>(
    cursor: Option<&Cursor>,
    query: &'a mut SeaCursor<SelectModel<E>>,
) -> &'a mut SeaCursor<SelectModel<E>> {
    let mut query = query;

    if cursor.is_none() {
        return query.first(DEFAULT_LIMIT as u64);
    }

    let cursor = cursor.unwrap();

    if let Some(after) = cursor.after() {
        query = query.after(after.value().to_owned());
    }

    if let Some(before) = cursor.before() {
        query = query.before(before.value().to_owned());
    }

    if let Some(first) = cursor.first() {
        query = query.first(first.value() as u64);
    }

    if let Some(last) = cursor.last() {
        query = query.last(last.value() as u64);
    }

    query
}
