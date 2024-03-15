use sea_orm::{DbBackend, EntityTrait, Select, SelectModel, SelectorRaw, Statement, Value};

use crate::common::domain::{
    criteria::{filter::Filter, Criteria},
    utils::sanitize_string,
};

pub const DEFAULT_LIMIT: usize = 10;

pub fn sea_convert_criteria<E: EntityTrait>(
    table_name: &str,
    query: &mut Select<E>,
    criteria: Criteria,
) -> SelectorRaw<SelectModel<E::Model>> {
    let (query_raw, query_values) = format_query(table_name, criteria);

    let statement = Statement::from_sql_and_values(DbBackend::Postgres, query_raw, query_values);
    query.clone().from_raw_sql(statement)
}

fn format_query(table_name: &str, criteria: Criteria) -> (String, Vec<Value>) {
    let mut where_raw = "".to_string();
    let mut order_raw = "created_at ASC".to_string();
    let mut query_values: Vec<Value> = Vec::new();
    let mut limit = DEFAULT_LIMIT;

    if criteria.has_filters() {
        let filter_len = criteria.filters().len() - 1;
        for (i, filter) in criteria.filters().iter().enumerate() {
            let current_query_values_len = query_values.len();
            let filter: &Filter = filter;
            let escaped_field = sanitize_string(filter.field().to_string().as_str());
            where_raw.push_str(
                format!(
                    "{} {} ${}",
                    escaped_field,
                    filter.operator().to_string(),
                    current_query_values_len + 1
                )
                .as_str(),
            );
            query_values.push(filter.value().to_string().into());

            if i < filter_len {
                where_raw.push_str(" AND ");
            }
        }
    }

    if let Some(cursor) = criteria.cursor() {
        if let Some(first) = cursor.first() {
            limit = first.value();
        } else if let Some(last) = cursor.last() {
            order_raw = "created_at DESC".to_string();
            limit = last.value();
        }

        if let Some(after) = cursor.after() {
            if !where_raw.is_empty() {
                where_raw.push_str(" AND ");
            }
            where_raw
                .push_str(format!("created_at > ${}::timestamp", query_values.len() + 1).as_str());
            query_values.push(after.to_string().into());
        }

        if let Some(before) = cursor.before() {
            if !where_raw.is_empty() {
                where_raw.push_str(" AND ");
            }
            where_raw
                .push_str(format!("created_at < ${}::timestamp", query_values.len() + 1).as_str());
            query_values.push(before.to_string().into());
        }
    }

    if let Some(order) = criteria.order() {
        let escaped_field = sanitize_string(order.order_by().to_string().as_str());
        order_raw
            .push_str(format!(", {} {}", escaped_field, order.order_type().to_string()).as_str());
    }

    if !where_raw.is_empty() {
        where_raw = format!("WHERE {}", where_raw);
    }

    (
        format!(
            "SELECT * FROM {} {} ORDER BY {} LIMIT {}",
            table_name, where_raw, order_raw, limit
        ),
        query_values,
    )
}

#[cfg(test)]
mod tests {
    use crate::common::domain::criteria::{
        cursor::{AfterCursor, BeforeCursor, Cursor, FirstField},
        filter::{FilterField, FilterOperator, FilterValue},
        order::{Order, OrderField, OrderType},
    };

    use super::*;

    #[test]
    fn test_convert_criteria() {
        let criteria = Criteria::new(
            vec![Filter::new(
                FilterField::try_from("email".to_string()).unwrap(),
                FilterOperator::Equal,
                FilterValue::try_from("admin@example.com".to_string()).unwrap(),
            )],
            Some(Order::new(
                OrderField::try_from("id".to_string()).unwrap(),
                OrderType::Asc,
            )),
            Some(Cursor::new(
                Some(AfterCursor::try_from("2024-01-01T00:00:00Z".to_string()).unwrap()),
                Some(BeforeCursor::try_from("2024-01-01T00:00:00Z".to_string()).unwrap()),
                Some(FirstField::try_from("15".to_string()).unwrap()),
                None,
            )),
        );

        let formatted_query = format_query("users", criteria);

        assert_eq!(
            formatted_query.0,
            "SELECT * FROM users WHERE email = $1 AND created_at > $2::timestamp AND created_at < $3::timestamp ORDER BY created_at ASC, id ASC LIMIT 15"
        );
        assert_eq!(formatted_query.1.len(), 3);
    }
}
