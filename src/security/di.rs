use redis::Client;
use security::session::{
    application::create::{
        command::{CreateSessionCommandHandler, CREATE_SESSION_COMMAND_TYPE},
        service::SessionCreator,
    },
    infrastructure::persistence::RedisSessionRepository,
};
use shared::{
    domain::bus::command::CommandBus,
    infrastructure::bus::{command::InMemoryCommandBus, query::InMemoryQueryBus},
};
use std::sync::Arc;

pub fn security_app_di(
    command_bus: &mut InMemoryCommandBus,
    query_bus: &mut InMemoryQueryBus,
    redis: &Client,
) {
    let redis_client = Arc::new(redis.clone());
    let session_repository = Arc::new(RedisSessionRepository::new(redis_client));

    let session_creator = SessionCreator::new(session_repository.clone());
    let session_creator_handler = Arc::new(CreateSessionCommandHandler::new(session_creator));

    command_bus.register_handler(CREATE_SESSION_COMMAND_TYPE, session_creator_handler);
}
