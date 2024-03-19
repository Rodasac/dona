use std::{collections::BTreeMap, time::Duration};

use async_graphql::{EmptySubscription, Schema};
use dona::{
    graphql::{DonaSchema, Mutation, Query},
    server::create_app,
};
use mockall::mock;
use poem::{
    middleware::AddDataEndpoint,
    session::{CookieConfig, ServerSession, SessionStorage},
    Result, Route,
};
use redis::{aio::MultiplexedConnection, Client as RedisClient};
use sea_orm::DatabaseConnection;
use security::session::domain::user::UserSession;
use serde_json::Value;
use time::OffsetDateTime;

pub const TEST_SESSION_ID: &str = "poem-session=BATz_xth_nsSbYDj5mTUJHfpVEOCiZefaKjJEUgTh14";

pub fn configure_app(
    db: DatabaseConnection,
    redis: RedisClient,
) -> AddDataEndpoint<
    AddDataEndpoint<AddDataEndpoint<Route, DatabaseConnection>, RedisClient>,
    DonaSchema,
> {
    create_app(
        db.clone(),
        redis.clone(),
        Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish(),
    )
}

pub async fn set_user_session(
    redis: RedisClient,
    is_admin: bool,
) -> ServerSession<MockSessionStorage> {
    let admin_session = UserSession::new(
        "user_id".to_string(),
        "session_id".to_string(),
        OffsetDateTime::now_utc(),
        is_admin,
    );

    let mut conn = redis.get_multiplexed_tokio_connection().await.unwrap();
    redis::cmd("SET")
        .arg("session:user_id:session_id")
        .arg(serde_json::to_string(&admin_session).unwrap())
        .query_async::<MultiplexedConnection, ()>(&mut conn)
        .await
        .unwrap();

    let mut mock = MockSessionStorage::new();
    mock.expect_load_session().returning(move |_| {
        Ok(Some(BTreeMap::from([
            (
                "user_id".to_string(),
                serde_json::to_value(admin_session.user_id()).unwrap(),
            ),
            (
                "session_id".to_string(),
                serde_json::to_value(admin_session.session_id()).unwrap(),
            ),
        ])))
    });
    mock.expect_update_session().returning(|_, _, _| Ok(()));
    mock.expect_remove_session().returning(|_| Ok(()));

    ServerSession::new(CookieConfig::default().secure(false), mock)
}

mock! {
    pub SessionStorage {}

    #[async_trait::async_trait]
    impl SessionStorage for SessionStorage {
        /// Load session entries.
        async fn load_session(&self, session_id: &str) -> Result<Option<BTreeMap<String, Value>>>;

        /// Insert or update a session.
        async fn update_session(
            &self,
            session_id: &str,
            entries: &BTreeMap<String, Value>,
            expires: Option<Duration>,
        ) -> Result<()>;

        /// Remove a session by session id.
        async fn remove_session(&self, session_id: &str) -> Result<()>;
    }
}
