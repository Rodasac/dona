use actix_web::web::ServiceConfig;
use async_graphql::{EmptySubscription, Schema};
use dona::{
    graphql::{Mutation, Query},
    server::create_app,
};
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

pub fn configure_app(
    db: DatabaseConnection,
    redis: RedisClient,
) -> impl FnOnce(&mut ServiceConfig) {
    create_app(
        db.clone(),
        redis.clone(),
        Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish(),
    )
}
