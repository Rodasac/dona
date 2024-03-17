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
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

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
                isAdmin: {},
                createdAt: "{}",
                updatedAt: "{}"
            }})
        }}
        "#,
        user.id().to_string(),
        user.email().to_string(),
        user.password().to_string(),
        user.full_name().to_string(),
        user.is_admin().to_string(),
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
            "INSERT INTO users (id, email, password, full_name, last_login, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', NULL, {}, '{}', '{}')",
             user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.is_admin().value(), user.created_at().to_string(), user.updated_at().to_string()
            ).as_str()
    ).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let password = "new_password";
    let full_name = "new_full_name";
    let is_admin = false;
    let updated_at = "2024-01-01T00:00:00Z";

    let query = format!(
        r#"
        mutation {{
            updateUser(input: {{
                id: "{}",
                password: "{}",
                fullName: "{}",
                isAdmin: {},
                updatedAt: "{}"
            }})
        }}
        "#,
        user.id().to_string(),
        password,
        full_name,
        is_admin,
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
            .format(&Rfc3339)
            .unwrap(),
        updated_at
    );
}

#[actix_web::test]
async fn test_backoffice_delete_user() {
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
            "INSERT INTO users (id, email, password, full_name, last_login, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', NULL, {}, '{}', '{}')",
             user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.is_admin().value(), user.created_at().to_string(), user.updated_at().to_string()
            ).as_str()
    ).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let query = format!(
        r#"
        mutation {{
            deleteUser(id: "{}")
        }}
        "#,
        user.id().to_string(),
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(r#"{"data":{"deleteUser":true}}"#,),
        "{:?}",
        response
    );

    let deleted_user = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM users WHERE id::text = $1",
            vec![user.id().to_string().into()],
        ))
        .await
        .unwrap();

    assert!(deleted_user.is_none());
}

#[actix_web::test]
async fn test_backoffice_find_user() {
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
            "INSERT INTO users (id, email, password, full_name, last_login, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', NULL, {}, '{}', '{}')",
             user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.is_admin().value(), user.created_at().to_string(), user.updated_at().to_string()
            ).as_str()
    ).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let query = format!(
        r#"
        query {{
            findUser(id: "{}") {{
                id
                email
                fullName
                createdAt
                updatedAt
            }}
        }}
        "#,
        user.id().to_string(),
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(format!(
            r#"{{"data":{{"findUser":{{"id":"{}","email":"{}","fullName":"{}","createdAt":"{}","updatedAt":"{}"}}}}}}"#,
            user.id().to_string(),
            user.email().to_string(),
            user.full_name().to_string(),
            user.created_at().to_string(),
            user.updated_at().to_string()
        )),
        "{:?}",
        response
    );
}

#[actix_web::test]
async fn test_backoffice_find_users() {
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
            "INSERT INTO users (id, email, password, full_name, last_login, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', NULL, {}, '{}', '{}')",
             user.id().to_string(), user.email().to_string(), user.password().to_string(), user.full_name().to_string(), user.is_admin().value(), user.created_at().to_string(), user.updated_at().to_string()
            ).as_str()
    ).await.unwrap();

    let test_server = test::init_service(App::new().configure(configure_app(db.clone()))).await;

    let query = format!(
        r#"
        query {{
            findUsers(criteria: {{
                filters: [{{
                    field: "email"
                    operator: EQUAL
                    value: "{}"
                }}]
                order: {{
                    orderBy: "email"
                    orderType: ASC
                }}
                cursor: {{
                    after: "{}"
                    before: "{}"
                    first: 1
                }}
            }}) {{
                id
                email
                fullName
                createdAt
                updatedAt
            }}
        }}
        "#,
        user.email().to_string(),
        user.created_at()
            .value()
            .checked_sub(Duration::hours(1))
            .unwrap_or(user.created_at().value().to_owned())
            .format(&Rfc3339)
            .unwrap(),
        user.updated_at()
            .value()
            .checked_add(Duration::hours(1))
            .unwrap_or(user.updated_at().value().to_owned())
            .format(&Rfc3339)
            .unwrap(),
    );

    let req = TestRequest::post()
        .uri("/graphql")
        .set_json(&json!({"query": query}))
        .to_request();
    let response = call_and_read_body(&test_server, req).await;

    assert_eq!(
        response,
        Bytes::from(format!(
            r#"{{"data":{{"findUsers":[{{"id":"{}","email":"{}","fullName":"{}","createdAt":"{}","updatedAt":"{}"}}]}}}}"#,
            user.id().to_string(),
            user.email().to_string(),
            user.full_name().to_string(),
            user.created_at().to_string(),
            user.updated_at().to_string()
        )),
        "{:?}",
        response
    );
}
