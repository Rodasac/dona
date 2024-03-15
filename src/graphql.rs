use async_graphql::{EmptySubscription, MergedObject, Object, Schema};

use crate::backoffice_app::graphql::{BackofficeMutation, BackofficeQuery};

#[derive(Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn version(&self) -> &str {
        "1.0"
    }
}

#[derive(MergedObject, Default)]
pub struct Query(BaseQuery, BackofficeQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(BackofficeMutation);

pub type DonaSchema = Schema<Query, Mutation, EmptySubscription>;
