pub mod backoffice_tests;

use actix_web::{
    test::{self, call_and_read_body, TestRequest},
    web::Bytes,
    App,
};
use sea_orm::Database;
use serde_json::json;

use crate::common::actix::configure_app;

#[actix_web::test]
async fn test_graphql_version() {
    use crate::common::db::get_db_image;

    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let query = format!(
        r#"
        query {{
            version
        }}
        "#,
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(r#"{"data":{"version":"1.0"}}"#,),
        "{:?}",
        response
    );
}
