use std::sync::Arc;

use crate::backoffice_app::di::backoffice_app_di;
use crate::graphql::{DonaSchema, Mutation, Query};
use crate::security::di::security_app_di;
use crate::{CommandBusType, QueryBusType};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::{AddDataEndpoint, CatchPanic, Cors};
use poem::session::{CookieConfig, RedisStorage, ServerSession, Session};
use poem::web::cookie::SameSite;
use poem::web::{Data, Html};
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};
use redis::Client as RedisClient;
use sea_orm::prelude::*;
use shared::infrastructure::bus::command::InMemoryCommandBus;
use shared::infrastructure::bus::query::InMemoryQueryBus;

#[handler]
async fn health_check() -> impl IntoResponse {
    "OK"
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[handler]
async fn index(
    schema: Data<&DonaSchema>,
    db: Data<&DatabaseConnection>,
    redis: Data<&RedisClient>,
    session: &Session,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;

    let mut command_bus = InMemoryCommandBus::default();
    let mut query_bus = InMemoryQueryBus::default();
    backoffice_app_di(&mut command_bus, &mut query_bus, &db);
    security_app_di(&mut command_bus, &redis);

    let command_bus: CommandBusType = Arc::new(command_bus);
    let query_bus: QueryBusType = Arc::new(query_bus);

    req = req
        .data(Arc::clone(&command_bus))
        .data(Arc::clone(&query_bus))
        .data(session.clone());

    schema.execute(req).await.into()
}

pub fn create_app(
    db: DatabaseConnection,
    redis: RedisClient,
    schema: DonaSchema,
) -> AddDataEndpoint<
    AddDataEndpoint<AddDataEndpoint<Route, DatabaseConnection>, RedisClient>,
    DonaSchema,
> {
    Route::new()
        .at("/graphql", get(index).post(index).options(index))
        .at("/", get(graphiql))
        .at("/health", get(health_check))
        .nest("/media", StaticFilesEndpoint::new("storage_files"))
        .data(db.clone())
        .data(redis.clone())
        .data(schema)
}

pub async fn run(db: &DatabaseConnection, redis: &RedisClient) -> Result<(), std::io::Error> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();
    let db_clone = db.clone();
    let redis_clone = redis.clone();

    let session_storage = ServerSession::new(
        CookieConfig::default()
            // Set the SameSite attribute to Lax on production
            .same_site(SameSite::None),
        RedisStorage::new(redis.get_connection_manager().await.unwrap()),
    );

    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(
            create_app(db_clone, redis_clone, schema)
                .with(CatchPanic::new())
                .with(Cors::new().allow_credentials(true).allow_origin_regex("*"))
                .with(session_storage),
        )
        .await
}
