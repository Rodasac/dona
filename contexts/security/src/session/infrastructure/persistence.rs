use std::sync::Arc;

use redis::Client;
use shared::domain::base_errors::BaseRepositoryError;

use crate::session::domain::{repository::SessionRepository, user::UserSession};

#[derive(Clone)]
pub struct RedisSessionRepository {
    pub conn: Arc<Client>,
}

impl RedisSessionRepository {
    pub fn new(conn: Arc<Client>) -> Self {
        Self { conn }
    }

    async fn get_conn(&self) -> Result<redis::aio::MultiplexedConnection, BaseRepositoryError> {
        self.conn
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))
    }
}

#[async_trait::async_trait]
impl SessionRepository for RedisSessionRepository {
    async fn get_with_user_id(
        &self,
        user_id: &str,
        session_id: &str,
    ) -> Result<UserSession, BaseRepositoryError> {
        let mut conn = self.get_conn().await?;
        let session: String = redis::cmd("GET")
            .arg(format!("session:{}:{}", user_id, session_id))
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        let session: UserSession = serde_json::from_str(&session)
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(session)
    }

    async fn get(&self, session_id: &str) -> Result<UserSession, BaseRepositoryError> {
        let mut conn = self.get_conn().await?;
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("session:*:{}", session_id))
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        let key = keys.first().ok_or(BaseRepositoryError::NotFound)?;
        let session: String = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        let session: UserSession = serde_json::from_str(&session)
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(session)
    }

    async fn save(&self, session: UserSession) -> Result<(), BaseRepositoryError> {
        let mut conn = self.get_conn().await?;
        let session_str = serde_json::to_string(&session)
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        redis::cmd("SET")
            .arg(format!(
                "session:{}:{}",
                session.user_id(),
                session.session_id()
            ))
            .arg(session_str)
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, session_id: &str) -> Result<(), BaseRepositoryError> {
        let mut conn = self.get_conn().await?;
        let key: Vec<String> = redis::cmd("KEYS")
            .arg(format!("session:*:{}", session_id))
            .query_async(&mut conn)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                BaseRepositoryError::NotFound
            })?;

        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        Ok(())
    }

    async fn delete_all(&self, user_id: &str) -> Result<(), BaseRepositoryError> {
        let mut conn = self.get_conn().await?;
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("session:{}:*", user_id))
            .query_async(&mut conn)
            .await
            .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;

        for key in keys {
            redis::cmd("DEL")
                .arg(key)
                .query_async(&mut conn)
                .await
                .map_err(|e| BaseRepositoryError::UnexpectedError(e.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use testcontainers::{clients::Cli as Docker, GenericImage};
    use time::OffsetDateTime;

    fn get_redis_image() -> GenericImage {
        GenericImage::new("redis", "7-alpine")
            .with_exposed_port(6379)
            .to_owned()
    }

    #[tokio::test]
    async fn should_get_conn() {
        let docker = Docker::default();
        let node = docker.run(get_redis_image());

        let conn = redis::Client::open(format!(
            "redis://localhost:{}",
            node.get_host_port_ipv4(6379)
        ))
        .unwrap();
        let conn = Arc::new(conn);

        let repository = RedisSessionRepository::new(conn);

        let conn = repository.get_conn().await;

        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn should_save_and_get_session() {
        let docker = Docker::default();
        let node = docker.run(get_redis_image());

        let conn = redis::Client::open(format!(
            "redis://localhost:{}",
            node.get_host_port_ipv4(6379)
        ))
        .unwrap();
        let conn = Arc::new(conn);

        let repository = RedisSessionRepository::new(conn);

        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            false,
        );

        repository.save(session.clone()).await.unwrap();

        let session = repository
            .get_with_user_id("user_id", "session_id")
            .await
            .unwrap();

        assert_eq!(session, session);
    }

    #[tokio::test]
    async fn should_delete_session() {
        let docker = Docker::default();
        let node = docker.run(get_redis_image());

        let conn = redis::Client::open(format!(
            "redis://localhost:{}",
            node.get_host_port_ipv4(6379)
        ))
        .unwrap();
        let conn = Arc::new(conn);

        let repository = RedisSessionRepository::new(conn);

        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            false,
        );

        repository.save(session.clone()).await.unwrap();

        repository.delete("session_id").await.unwrap();

        let session = repository.get_with_user_id("user_id", "session_id").await;

        assert!(session.is_err(), "{:?}", session);
    }

    #[tokio::test]
    async fn should_delete_all_sessions() {
        let docker = Docker::default();
        let node = docker.run(get_redis_image());

        let conn = redis::Client::open(format!(
            "redis://localhost:{}",
            node.get_host_port_ipv4(6379)
        ))
        .unwrap();
        let conn = Arc::new(conn);

        let repository = RedisSessionRepository::new(conn);

        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            false,
        );

        repository.save(session.clone()).await.unwrap();

        repository.delete_all("user_id").await.unwrap();

        let session = repository.get_with_user_id("user_id", "session_id").await;

        assert!(session.is_err());
    }

    #[tokio::test]
    async fn should_get_session() {
        let docker = Docker::default();
        let node = docker.run(get_redis_image());

        let conn = redis::Client::open(format!(
            "redis://localhost:{}",
            node.get_host_port_ipv4(6379)
        ))
        .unwrap();
        let conn = Arc::new(conn);

        let repository = RedisSessionRepository::new(conn);

        let session = UserSession::new(
            "user_id".to_string(),
            "session_id".to_string(),
            OffsetDateTime::now_utc(),
            false,
        );

        repository.save(session.clone()).await.unwrap();

        let session = repository.get("session_id").await.unwrap();

        assert_eq!(session, session);
    }
}
