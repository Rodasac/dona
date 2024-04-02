use std::sync::Arc;

use dona_context::{
    dona::infrastructure::persistence::sea_dona_repo::SeaDonaRepo,
    posts::infrastructure::persistence::sea_post_repo::SeaPostRepo,
    user_payment_method::{
        application::{
            create::{
                command::{
                    CreateUserPaymentMethodCommandHandler, CREATE_USER_PAYMENT_METHOD_COMMAND_TYPE,
                },
                service::UserPaymentMethodCreator,
            },
            delete::{
                command::{
                    DeleteUserPaymentMethodCommandHandler, DELETE_USER_PAYMENT_METHOD_COMMAND_TYPE,
                },
                service::UserPaymentMethodDeleter,
            },
            find::{
                query::{FindUserPaymentMethodQueryHandler, FIND_USER_PAYMENT_METHOD_QUERY_TYPE},
                service::UserPaymentMethodFinder,
            },
            find_by_criteria::{
                query::{
                    FindUserPaymentMethodsQueryHandler,
                    FIND_USER_PAYMENT_METHODS_BY_CRITERIA_QUERY_TYPE,
                },
                service::UserPaymentMethodsFinder,
            },
            get_user_payment_methods::{
                query::{GetUserPaymentMethodsQueryHandler, GET_USER_PAYMENT_METHODS_QUERY_TYPE},
                service::GetPaymentMethodsByUser,
            },
            update_instructions::{
                command::{
                    UpdateUserPaymentMethodInstructionsCommandHandler,
                    UPDATE_USER_PAYMENT_METHOD_INSTRUCTIONS_COMMAND_TYPE,
                },
                service::UserPaymentMethodInstructionsUpdater,
            },
        },
        infrastructure::persistence::sea_user_payment_method_repo::SeaUserPaymentMethodRepo,
    },
};
use sea_orm::DatabaseConnection;
use shared::{
    domain::bus::{command::CommandBus, query::QueryBus},
    infrastructure::bus::{
        command::InMemoryCommandBus, event::InMemoryEventBus, query::InMemoryQueryBus,
    },
};

/// This function is used to register all the dependencies of the dona app.
///
/// The event bus must be injected as an Arc because it is shared between the services
/// and the initialization must be done before injecting it into the services
pub fn dona_app_di(
    command_bus: &mut InMemoryCommandBus,
    query_bus: &mut InMemoryQueryBus,
    event_bus: Arc<InMemoryEventBus>,
    db: &DatabaseConnection,
) {
    // User Payment Method

    let user_payment_method_repository = Arc::new(SeaUserPaymentMethodRepo::new(db.clone()));

    let create_user_payment_method =
        UserPaymentMethodCreator::new(user_payment_method_repository.clone(), event_bus.clone());
    let create_user_payment_method_command_handler =
        CreateUserPaymentMethodCommandHandler::new(create_user_payment_method);

    let update_instructions = UserPaymentMethodInstructionsUpdater::new(
        user_payment_method_repository.clone(),
        event_bus.clone(),
    );
    let update_instructions_command_handler =
        UpdateUserPaymentMethodInstructionsCommandHandler::new(update_instructions);

    let delete_user_payment_method =
        UserPaymentMethodDeleter::new(user_payment_method_repository.clone(), event_bus.clone());
    let delete_user_payment_method_command_handler =
        DeleteUserPaymentMethodCommandHandler::new(delete_user_payment_method);

    command_bus.register_handler(
        CREATE_USER_PAYMENT_METHOD_COMMAND_TYPE,
        Arc::new(create_user_payment_method_command_handler),
    );
    command_bus.register_handler(
        UPDATE_USER_PAYMENT_METHOD_INSTRUCTIONS_COMMAND_TYPE,
        Arc::new(update_instructions_command_handler),
    );
    command_bus.register_handler(
        DELETE_USER_PAYMENT_METHOD_COMMAND_TYPE,
        Arc::new(delete_user_payment_method_command_handler),
    );

    let find_user_payment_methods =
        UserPaymentMethodsFinder::new(user_payment_method_repository.clone());
    let find_user_payment_methods_query_handler =
        FindUserPaymentMethodsQueryHandler::new(find_user_payment_methods);

    let find_user_payment_method =
        UserPaymentMethodFinder::new(user_payment_method_repository.clone());
    let find_user_payment_method_query_handler =
        FindUserPaymentMethodQueryHandler::new(find_user_payment_method);

    let get_payment_methods_by_user =
        GetPaymentMethodsByUser::new(user_payment_method_repository.clone());
    let get_payment_methods_by_user_query_handler =
        GetUserPaymentMethodsQueryHandler::new(get_payment_methods_by_user);

    query_bus.register_handler(
        FIND_USER_PAYMENT_METHODS_BY_CRITERIA_QUERY_TYPE,
        Arc::new(find_user_payment_methods_query_handler),
    );
    query_bus.register_handler(
        FIND_USER_PAYMENT_METHOD_QUERY_TYPE,
        Arc::new(find_user_payment_method_query_handler),
    );
    query_bus.register_handler(
        GET_USER_PAYMENT_METHODS_QUERY_TYPE,
        Arc::new(get_payment_methods_by_user_query_handler),
    );

    // Dona
    let _dona_repository = Arc::new(SeaDonaRepo::new(db.clone()));

    // Posts
    let _posts_repository = Arc::new(SeaPostRepo::new(db.clone()));
}
