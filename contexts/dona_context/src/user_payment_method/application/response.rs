use serde::{Deserialize, Serialize};
use shared::domain::bus::query::Response;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserPaymentMethodResponse {
    pub id: String,
    pub user_id: String,
    pub payment_method: String,
    pub instructions: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Response for UserPaymentMethodResponse {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserPaymentMethodsResponse {
    pub users: Vec<UserPaymentMethodResponse>,
}

impl Response for UserPaymentMethodsResponse {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
