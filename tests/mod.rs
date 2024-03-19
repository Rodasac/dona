use migration::{Migrator, MigratorTrait};
use poem::{test::TestClient, EndpointExt};
use sea_orm::Database;

use crate::common::{
    db::{get_db_image, get_redis_image},
    poem::{configure_app, set_user_session},
};

mod common;
mod graphql;

#[tokio::test]
async fn health_check_should_return_200() {
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
            .with(set_user_session(redis_client.clone(), false).await),
    );

    let response = test_server.get("/health").send().await;
    response.assert_status_is_ok();
}
