use std::sync::Arc;

use backoffice::auth::{
    application::{
        authenticate::{
            command::{AuthenticateUserCommandHandler, AUTHENTICATE_USER_COMMAND_TYPE},
            service::UserAuthenticator,
        },
        create_user::{
            command::{CreateUserCommandHandler, CREATE_USER_COMMAND_TYPE},
            service::CreateUser,
        },
        delete_user::{
            command::{DeleteUserCommandHandler, DELETE_USER_COMMAND_TYPE},
            service::UserDeleter,
        },
        find_user::{
            query::{FindUserByIdQueryHandler, FIND_USER_BY_ID_QUERY_TYPE},
            service::UserFinder,
        },
        find_users_by_criteria::{
            query::{FindUsersByCriteriaQueryHandler, FIND_USERS_BY_CRITERIA_QUERY_TYPE},
            service::UsersFinderByCriteria,
        },
        update_user::{
            command::{UpdateUserCommandHandler, UPDATE_USER_COMMAND_TYPE},
            service::UpdateUser,
        },
    },
    infrastructure::{
        hasher::argon_hasher::ArgonHasher, persistence::sea_user_repo::SeaUserRepository,
    },
};
use sea_orm::DatabaseConnection;
use shared::{
    domain::bus::{command::CommandBus, query::QueryBus},
    infrastructure::bus::{command::InMemoryCommandBus, query::InMemoryQueryBus},
};

pub type CommandBusType = Arc<dyn CommandBus>;
pub type QueryBusType = Arc<dyn QueryBus>;

pub fn backoffice_app_di(
    command_bus: &mut InMemoryCommandBus,
    query_bus: &mut InMemoryQueryBus,
    db: &DatabaseConnection,
) {
    let user_repository = Arc::new(SeaUserRepository::new(db.clone()));
    let password_hasher = Arc::new(ArgonHasher::default());

    let create_user = CreateUser::new(user_repository.clone(), password_hasher.clone());
    let create_user_command_handler = CreateUserCommandHandler::new(create_user);

    let update_user = UpdateUser::new(user_repository.clone(), password_hasher.clone());
    let update_user_command_handler = UpdateUserCommandHandler::new(update_user);

    let delete_user = UserDeleter::new(user_repository.clone());
    let delete_user_command_handler = DeleteUserCommandHandler::new(delete_user);

    let authenticator_service =
        UserAuthenticator::new(user_repository.clone(), password_hasher.clone());
    let authenticator_command_handler = AuthenticateUserCommandHandler::new(authenticator_service);

    command_bus.register_handler(
        CREATE_USER_COMMAND_TYPE,
        Arc::new(create_user_command_handler),
    );
    command_bus.register_handler(
        UPDATE_USER_COMMAND_TYPE,
        Arc::new(update_user_command_handler),
    );
    command_bus.register_handler(
        DELETE_USER_COMMAND_TYPE,
        Arc::new(delete_user_command_handler),
    );
    command_bus.register_handler(
        AUTHENTICATE_USER_COMMAND_TYPE,
        Arc::new(authenticator_command_handler),
    );

    let find_user_by_id_query_handler =
        FindUserByIdQueryHandler::new(UserFinder::new(user_repository.clone()));
    let find_users_by_criteria_query_handler =
        FindUsersByCriteriaQueryHandler::new(UsersFinderByCriteria::new(user_repository.clone()));

    query_bus.register_handler(
        FIND_USER_BY_ID_QUERY_TYPE,
        Arc::new(find_user_by_id_query_handler),
    );
    query_bus.register_handler(
        FIND_USERS_BY_CRITERIA_QUERY_TYPE,
        Arc::new(find_users_by_criteria_query_handler),
    );
}
