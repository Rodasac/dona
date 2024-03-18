use dona::server::run;
use sea_orm::Database;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("postgres://dona:dona@localhost:5432/dona")
        .await
        .expect("Failed to connect to database");
    let redis = redis::Client::open("redis://localhost/").expect("Failed to connect to redis");

    run(&db, &redis).await
}
