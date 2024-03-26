use backoffice::auth::{
    domain::{password_hasher::UserPasswordHasher, user::tests::UserMother},
    infrastructure::hasher::argon_hasher::ArgonHasher,
};
use migration::{Migrator, MigratorTrait};
use poem::{
    session::{CookieConfig, RedisStorage, ServerSession},
    test::TestClient,
    web::cookie::SameSite,
    EndpointExt,
};
use sea_orm::{ConnectionTrait, Database, Statement};
use serde_json::json;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

use crate::common::{
    db::get_db_image,
    poem::{set_user_session, TEST_SESSION_ID},
};
use crate::common::{db::get_redis_image, poem::configure_app};

// TODO: Find a way to test file uploads in GraphQL with Poem (multipart/form-data)
// With the Altair GraphQL Client and Apollo Studio, it works.
#[tokio::test]
async fn test_backoffice_create_user() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();

    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), true).await),
    );

    let user = UserMother::random();

    let query = format!(
        r#"
        mutation {{
            createUser(input: {{
                id: "{}",
                username: "{}",
                email: "{}",
                password: "{}",
                fullName: "{}",
                createdAt: "{}",
                updatedAt: "{}"
            }})
        }}
        "#,
        user.id().to_string(),
        user.username().to_string(),
        user.email().to_string(),
        user.password().to_string(),
        user.full_name().to_string(),
        user.created_at_string(),
        user.updated_at_string()
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", TEST_SESSION_ID)
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "createUser": true
        }
    }))
    .await;
}

// TODO: Find a way to test file uploads in GraphQL with Poem (multipart/form-data)
// With the Altair GraphQL Client and Apollo Studio, it works.
#[tokio::test]
async fn test_backoffice_update_user() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user = UserMother::random();
    db.execute_unprepared(
        format!(
            r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
             user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL".to_string()), user.is_admin(), user.created_at(), user.updated_at()
            ).as_str()
    ).await.unwrap();

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), true).await),
    );

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

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", TEST_SESSION_ID)
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "updateUser": true
        }
    }))
    .await;

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
            password,
            updated_user
                .try_get::<String>("", "password")
                .unwrap()
                .as_str(),
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

#[tokio::test]
async fn test_backoffice_delete_user() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user = UserMother::random();
    db.execute_unprepared(
        format!(
            r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
             user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL".to_string()), user.is_admin(), user.created_at(), user.updated_at()
            ).as_str()
    ).await.unwrap();

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), false).await),
    );

    let query = format!(
        r#"
        mutation {{
            deleteUser(id: "{}")
        }}
        "#,
        user.id().to_string(),
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", TEST_SESSION_ID)
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "deleteUser": true
        }
    }))
    .await;

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

#[tokio::test]
async fn test_backoffice_find_user() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user = UserMother::random();
    db.execute_unprepared(
        format!(
            r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
             user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL".to_string()), user.is_admin(), user.created_at(), user.updated_at()
            ).as_str()
    ).await.unwrap();

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), false).await),
    );

    let query = format!(
        r#"
        query {{
            findUser(id: "{}") {{
                id
                username
                email
                fullName
                profilePicture
                createdAt
                updatedAt
            }}
        }}
        "#,
        user.id().to_string(),
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", TEST_SESSION_ID)
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "findUser": {
                "id": user.id().to_string(),
                "username": user.username().to_string(),
                "email": user.email().to_string(),
                "fullName": user.full_name().to_string(),
                "profilePicture": user.profile_picture().map(|v| v.to_owned()),
                "createdAt": user.created_at_string(),
                "updatedAt": user.updated_at_string()
            }
        }
    }))
    .await;
}

#[tokio::test]
async fn test_backoffice_find_users() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user = UserMother::random();
    db.execute_unprepared(
        format!(
            r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
             user.id(), user.username(), user.email(), user.password(), user.full_name(), user.profile_picture().unwrap_or("NULL".to_string()), user.is_admin(), user.created_at(), user.updated_at()
            ).as_str()
    ).await.unwrap();

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), false).await),
    );

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
                username
                email
                fullName
                profilePicture
                createdAt
                updatedAt
            }}
        }}
        "#,
        user.email().to_string(),
        user.created_at()
            .checked_sub(Duration::hours(1))
            .unwrap_or(user.created_at().to_owned())
            .format(&Rfc3339)
            .unwrap(),
        user.updated_at()
            .checked_add(Duration::hours(1))
            .unwrap_or(user.updated_at().to_owned())
            .format(&Rfc3339)
            .unwrap(),
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", TEST_SESSION_ID)
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "findUsers": [{
                "id": user.id().to_string(),
                "username": user.username().to_string(),
                "email": user.email().to_string(),
                "fullName": user.full_name().to_string(),
                "profilePicture": user.profile_picture().map(|v| v.to_owned()),
                "createdAt": user.created_at_string(),
                "updatedAt": user.updated_at_string()
            }]
        }
    }))
    .await;
}

#[tokio::test]
async fn test_backoffice_login_and_logout() {
    let docker = testcontainers::clients::Cli::default();
    let image = get_db_image();
    let db_image = docker.run(image);
    let port = db_image.get_host_port_ipv4(5432);
    println!("Postgres running on port: {}", port);

    let redis = docker.run(get_redis_image());
    let redis_port = redis.get_host_port_ipv4(6379);
    let redis_url = format!("redis://localhost:{port}/", port = redis_port);
    println!("Redis running on port: {}", redis_port);
    let redis_client = redis::Client::open(redis_url).unwrap();

    let db = Database::connect(format!("postgres://dona:dona@localhost:{port}/dona_test"))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user = UserMother::random();
    let password = "password";
    let hashed_password = ArgonHasher::default().hash(password).unwrap();
    db.execute_unprepared(
        format!(
            r#"INSERT INTO users (id, username, email, password, full_name, last_login, profile_picture, is_admin, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', NULL, '{}', {}, '{}', '{}')"#,
             user.id(), user.username(), user.email(), hashed_password, user.full_name(), user.profile_picture().unwrap_or("NULL".to_string()), user.is_admin(), user.created_at(), user.updated_at()
            ).as_str()
    ).await.unwrap();

    let session_storage = ServerSession::new(
        CookieConfig::default()
            // Set the SameSite attribute to Lax on production
            .same_site(SameSite::None),
        RedisStorage::new(redis_client.get_connection_manager().await.unwrap()),
    );
    let test_server =
        TestClient::new(configure_app(db.clone(), redis_client.clone()).with(session_storage));

    // Test login

    let query = format!(
        r#"
        mutation {{
            login(input: {{
                email: "{}",
                password: "{}"
            }})
        }}
        "#,
        user.email().to_string(),
        password
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .send()
        .await;
    req.assert_status_is_ok();

    req.assert_json(json!({
        "data": {
            "login": true
        }
    }))
    .await;

    let user_session: Vec<String> = redis::cmd("KEYS")
        .arg(format!("session:{}:*", user.id().to_string()))
        .query_async(&mut redis_client.get_tokio_connection().await.unwrap())
        .await
        .unwrap();

    assert_eq!(user_session.len(), 1);

    // Test logout

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .send()
        .await;
    let set_cookie = req.0.headers().get("set-cookie").unwrap().to_str().unwrap();
    let cookie = set_cookie.split(';').next().unwrap();

    let query = r#"
        mutation {
            logout
        }
    "#;

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .header("Cookie", cookie)
        .send()
        .await;

    req.assert_status_is_ok();
    req.assert_json(json!({
        "data": {
            "logout": true
        }
    }))
    .await;

    let user_session: Vec<String> = redis::cmd("KEYS")
        .arg(format!("session:{}:*", user.id().to_string()))
        .query_async(&mut redis_client.get_tokio_connection().await.unwrap())
        .await
        .unwrap();

    // Only the first session remains
    assert_eq!(user_session.len(), 1);
}
