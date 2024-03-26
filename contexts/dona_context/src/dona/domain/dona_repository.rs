use shared::domain::{base_errors::BaseRepositoryError, criteria::Criteria};

use super::dona::{Dona, DonaId};

#[async_trait::async_trait]
pub trait DonaRepository: Send + Sync {
    async fn find_by_id(&self, id: DonaId) -> Result<Dona, BaseRepositoryError>;
    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Dona>, BaseRepositoryError>;
    async fn find_all(&self) -> Result<Vec<Dona>, BaseRepositoryError>;
    async fn save(&self, dona: &Dona) -> Result<(), BaseRepositoryError>;
    async fn delete(&self, id: DonaId) -> Result<(), BaseRepositoryError>;
}

#[cfg(test)]
pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub DonaRepository {}

        #[async_trait::async_trait]
        impl DonaRepository for DonaRepository {
            async fn find_by_id(&self, id: DonaId) -> Result<Dona, BaseRepositoryError>;
            async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Dona>, BaseRepositoryError>;
            async fn find_all(&self) -> Result<Vec<Dona>, BaseRepositoryError>;
            async fn save(&self, dona: &Dona) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, id: DonaId) -> Result<(), BaseRepositoryError>;
        }
    }
}
