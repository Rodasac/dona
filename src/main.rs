use dona::server::run;
use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("postgres://dona:dona@localhost:5432/dona")
        .await
        .expect("Failed to connect to database");

    run(&db).await
}
