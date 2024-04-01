use shared::domain::{base_errors::BaseRepositoryError, criteria::Criteria};

use super::user_payment_method::{UserPaymentMethod, UserPaymentMethodId};

#[async_trait::async_trait]
pub trait UserPaymentMethodRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: UserPaymentMethodId,
    ) -> Result<UserPaymentMethod, BaseRepositoryError>;
    async fn find_by_criteria(
        &self,
        criteria: Criteria,
    ) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError>;
    async fn find_all(&self) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError>;
    async fn save(
        &self,
        user_payment_method: &UserPaymentMethod,
    ) -> Result<(), BaseRepositoryError>;
    async fn delete(&self, id: UserPaymentMethodId) -> Result<(), BaseRepositoryError>;
}

#[cfg(test)]
pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub UserPaymentMethodRepository {}

        #[async_trait::async_trait]
        impl UserPaymentMethodRepository for UserPaymentMethodRepository {
            async fn find_by_id(&self, id: UserPaymentMethodId) -> Result<UserPaymentMethod, BaseRepositoryError>;
            async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError>;
            async fn find_all(&self) -> Result<Vec<UserPaymentMethod>, BaseRepositoryError>;
            async fn save(&self, user_payment_method: &UserPaymentMethod) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, id: UserPaymentMethodId) -> Result<(), BaseRepositoryError>;
        }
    }
}
