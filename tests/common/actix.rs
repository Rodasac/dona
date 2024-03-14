use actix_web::web::ServiceConfig;
use async_graphql::{EmptySubscription, Schema};
use dona::{
    graphql::{Mutation, Query},
    server::create_app,
};
use sea_orm::DatabaseConnection;

pub fn configure_app(db: DatabaseConnection) -> impl FnOnce(&mut ServiceConfig) {
    create_app(
        db.clone(),
        Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish(),
    )
}
