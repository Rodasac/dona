pub mod backoffice_tests;
use poem::{test::TestClient, EndpointExt};
use sea_orm::Database;
use serde_json::json;

use crate::common::{
    db::{get_db_image, get_redis_image},
    poem::{configure_app, set_user_session},
};

#[tokio::test]
async fn test_graphql_version() {
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

    let test_server = TestClient::new(
        configure_app(db.clone(), redis_client.clone())
            .with(set_user_session(redis_client.clone(), false).await),
    );

    let query = format!(
        r#"
        query {{
            version
        }}
        "#,
    );

    let req = test_server
        .post("/graphql")
        .body_json(&json!({"query": query}))
        .send()
        .await;

    req.assert_status_is_ok();
    req.assert_json(json!({
        "data": {
            "version": "1.0"
        }
    }))
    .await;
}
