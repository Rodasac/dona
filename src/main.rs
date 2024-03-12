mod backoffice_app;
mod graphql;

use std::sync::Arc;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use backoffice_app::di::{backoffice_app_di, BackofficeCommandBusType};
use graphql::{DonaSchema, Mutation, Query};
use sea_orm::{prelude::*, Database};
use shared::common::infrastructure::bus::command::InMemoryCommandBus;

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
    let mut bus = InMemoryCommandBus::default();
    backoffice_app_di(&mut bus, &db);

    let bus: BackofficeCommandBusType = Arc::new(bus);

    let request = gql_request.into_inner().data(Arc::clone(&bus));

    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("postgres://dona:dona@localhost:5432/dona")
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        let schema =
            Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

        App::new()
            .app_data(web::Data::new(schema))
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
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
