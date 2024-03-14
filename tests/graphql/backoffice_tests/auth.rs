use actix_web::{
    test::{self, call_and_read_body, TestRequest},
    web::Bytes,
    App,
};
use backoffice::auth::{
    domain::{password_hasher::UserPasswordHasher, user::tests::UserMother},
    infrastructure::hasher::argon_hasher::ArgonHasher,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, Statement};
use serde_json::json;
use time::{format_description::well_known::Iso8601, OffsetDateTime};

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

#[actix_web::test]
async fn test_backoffice_update_user() {
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

    let user = UserMother::random();
    db.execute_unprepared(
        format!(
            "INSERT INTO users (id, email, password, full_name, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
             user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.created_at().to_string(), user.updated_at().to_string()
            ).as_str()
    ).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let password = "new_password";
    let full_name = "new_full_name";
    let updated_at = "2024-01-01T00:00:00.000000000Z";

    let query = format!(
        r#"
        mutation {{
            updateUser(input: {{
                id: "{}",
                password: "{}",
                fullName: "{}",
                updatedAt: "{}"
            }})
        }}
        "#,
        user.id().to_string(),
        password,
        full_name,
        updated_at
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(r#"{"data":{"updateUser":true}}"#,),
        "{:?}",
        response
    );

    let updated_user = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM users WHERE id::text = $1",
            vec![user.id().to_string().into()],
        ))
        .await
        .unwrap();

    assert!(updated_user.is_some());
    let updated_user = updated_user.unwrap();
    ArgonHasher::default()
        .verify(
            updated_user
                .try_get::<String>("", "password")
                .unwrap()
                .as_str(),
            password,
        )
        .unwrap();
    assert_eq!(
        updated_user.try_get::<String>("", "full_name").unwrap(),
        full_name
    );
    assert_eq!(
        updated_user
            .try_get::<OffsetDateTime>("", "updated_at")
            .unwrap()
            .format(&Iso8601::DEFAULT)
            .unwrap(),
        updated_at
    );
}
