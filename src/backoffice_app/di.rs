use std::sync::Arc;

use backoffice::auth::{
    application::{
        create_user::{
            command::{CreateUserCommandHandler, CREATE_USER_COMMAND_TYPE},
            service::CreateUser,
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
use shared::common::{
    domain::bus::command::CommandBus, infrastructure::bus::command::InMemoryCommandBus,
};

pub type BackofficeCommandBusType = Arc<dyn CommandBus>;

pub fn backoffice_app_di(bus: &mut InMemoryCommandBus, db: &DatabaseConnection) {
    let user_repository = Arc::new(SeaUserRepository::new(db.clone()));
    let password_hasher = Arc::new(ArgonHasher::default());

    let create_user = CreateUser::new(user_repository.clone(), password_hasher.clone());
    let create_user_command_handler = CreateUserCommandHandler::new(create_user);

    let update_user = UpdateUser::new(user_repository.clone(), password_hasher.clone());
    let update_user_command_handler = UpdateUserCommandHandler::new(update_user);

    bus.register_handler(
        CREATE_USER_COMMAND_TYPE,
        Arc::new(create_user_command_handler),
    );
    bus.register_handler(
        UPDATE_USER_COMMAND_TYPE,
        Arc::new(update_user_command_handler),
    );
}
