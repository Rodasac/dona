use std::io::Error;
use std::sync::Arc;

use crate::backoffice_app::di::{backoffice_app_di, CommandBusType, QueryBusType};
use crate::graphql::{DonaSchema, Mutation, Query};
use actix_web::web::ServiceConfig;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::prelude::*;
use shared::common::infrastructure::bus::command::InMemoryCommandBus;
use shared::common::infrastructure::bus::query::InMemoryQueryBus;

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/graphql")
                .subscription_endpoint("/ws")
                .finish(),
        )
}

async fn index(
    schema: web::Data<DonaSchema>,
    db: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut command_bus = InMemoryCommandBus::default();
    let mut query_bus = InMemoryQueryBus::default();
    backoffice_app_di(&mut command_bus, &mut query_bus, &db);

    let command_bus: CommandBusType = Arc::new(command_bus);
    let query_bus: QueryBusType = Arc::new(query_bus);

    let request = gql_request
        .into_inner()
        .data(Arc::clone(&command_bus))
        .data(Arc::clone(&query_bus));

    schema.execute(request).await.into()
}

pub fn create_app(db: DatabaseConnection, schema: DonaSchema) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(db.clone()))
            .service(
                web::resource("/graphql")
                    .guard(
                        guard::Any(guard::Post())
                            .or(guard::Get())
                            .or(guard::Head())
                            .or(guard::Options()),
                    )
                    .to(index),
            )
            .service(web::resource("/").guard(guard::Get()).to(graphiql));
    }
}

pub async fn run(db: &DatabaseConnection) -> Result<(), Error> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();
    let db_clone = db.clone();

    HttpServer::new(move || App::new().configure(create_app(db_clone.clone(), schema.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}