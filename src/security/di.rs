use redis::Client;
use security::session::{
    application::{
        check_is_authenticated::{
            command::{CheckIsAuthenticatedCommandHandler, CHECK_IS_AUTHENTICATED_COMMAND_TYPE},
            service::IsAuthenticatedChecker,
        },
        check_permission::{
            command::{CheckPermissionCommandHandler, CHECK_PERMISSION_COMMAND_TYPE},
            service::PermissionChecker,
        },
        create::{
            command::{CreateSessionCommandHandler, CREATE_SESSION_COMMAND_TYPE},
            service::SessionCreator,
        },
        logout::{
            command::{LogoutSessionCommandHandler, LOGOUT_SESSION_COMMAND_TYPE},
            service::SessionLogout,
        },
    },
    infrastructure::persistence::RedisSessionRepository,
};
use shared::{domain::bus::command::CommandBus, infrastructure::bus::command::InMemoryCommandBus};
use std::sync::Arc;

pub fn security_app_di(command_bus: &mut InMemoryCommandBus, redis: &Client) {
    let redis_client = Arc::new(redis.clone());
    let session_repository = Arc::new(RedisSessionRepository::new(redis_client));

    let session_creator = SessionCreator::new(session_repository.clone());
    let session_creator_handler = Arc::new(CreateSessionCommandHandler::new(session_creator));

    let session_logout = SessionLogout::new(session_repository.clone());
    let session_logout_handler = Arc::new(LogoutSessionCommandHandler::new(session_logout));

    let session_permission_checker = PermissionChecker::new(session_repository.clone());
    let session_permission_checker_handler = Arc::new(CheckPermissionCommandHandler::new(
        session_permission_checker,
    ));

    let session_is_authenticated_checker = IsAuthenticatedChecker::new(session_repository.clone());
    let session_is_authenticated_checker_handler = Arc::new(
        CheckIsAuthenticatedCommandHandler::new(session_is_authenticated_checker),
    );

    command_bus.register_handler(CREATE_SESSION_COMMAND_TYPE, session_creator_handler);
    command_bus.register_handler(LOGOUT_SESSION_COMMAND_TYPE, session_logout_handler);
    command_bus.register_handler(
        CHECK_PERMISSION_COMMAND_TYPE,
        session_permission_checker_handler,
    );
    command_bus.register_handler(
        CHECK_IS_AUTHENTICATED_COMMAND_TYPE,
        session_is_authenticated_checker_handler,
    );
}
