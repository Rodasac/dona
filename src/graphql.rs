use async_graphql::{EmptySubscription, MergedObject, Object, Schema};

use crate::backoffice_app::graphql::BackofficeMutation;

#[derive(Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn version(&self) -> &str {
        "1.0"
    }
}

#[derive(MergedObject, Default)]
pub struct Query(BaseQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(BackofficeMutation);

pub type DonaSchema = Schema<Query, Mutation, EmptySubscription>;
