use testcontainers::{core::WaitFor, GenericImage};

pub fn get_db_image() -> GenericImage {
    GenericImage::new("postgres", "16-alpine")
        .with_env_var("POSTGRES_USER", "dona")
        .with_env_var("POSTGRES_PASSWORD", "dona")
        .with_env_var("POSTGRES_DB", "dona_test")
        .with_exposed_port(5432)
        .with_wait_for(WaitFor::millis(5000))
        .to_owned()
}
