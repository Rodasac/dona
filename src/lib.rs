use std::sync::Arc;

use shared::domain::bus::{command::CommandBus, query::QueryBus};

pub mod backoffice_app;
pub mod graphql;
pub mod security;
pub mod server;

pub type CommandBusType = Arc<dyn CommandBus>;
pub type QueryBusType = Arc<dyn QueryBus>;

pub const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 5;
