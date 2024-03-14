use actix_web::{
    test::{self, call_and_read_body, TestRequest},
    web::Bytes,
    App,
};
use backoffice::auth::domain::user::tests::UserMother;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use serde_json::json;

use crate::common::actix::configure_app;

#[actix_web::test]
async fn test_backoffice_create_user() {
    use crate::common::db::get_db_image;

    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let user = UserMother::random();

    let query = format!(
        r#"
        mutation {{
            createUser(input: {{
                id: "{}",
                email: "{}",
                password: "{}",
                fullName: "{}",
                createdAt: "{}",
                updatedAt: "{}"
            }})
        }}
        "#,
        user.id().to_string(),
        user.email().to_string(),
        user.password().to_string(),
        user.full_name().to_string(),
        user.created_at().to_string(),
        user.updated_at().to_string()
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(r#"{"data":{"createUser":true}}"#,),
        "{:?}",
        response
    );
}
