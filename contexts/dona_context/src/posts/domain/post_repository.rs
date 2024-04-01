use shared::domain::{base_errors::BaseRepositoryError, criteria::Criteria};

use super::post::{Post, PostId};

#[async_trait::async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: PostId) -> Result<Post, BaseRepositoryError>;
    async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Post>, BaseRepositoryError>;
    async fn find_all(&self) -> Result<Vec<Post>, BaseRepositoryError>;
    async fn save(&self, post: &Post) -> Result<(), BaseRepositoryError>;
    async fn delete(&self, id: PostId) -> Result<(), BaseRepositoryError>;
}

#[cfg(test)]
pub mod tests {
    use mockall::mock;

    use super::*;

    mock! {
        pub PostRepository {}

        #[async_trait::async_trait]
        impl PostRepository for PostRepository {
            async fn find_by_id(&self, id: PostId) -> Result<Post, BaseRepositoryError>;
            async fn find_by_criteria(&self, criteria: Criteria) -> Result<Vec<Post>, BaseRepositoryError>;
            async fn find_all(&self) -> Result<Vec<Post>, BaseRepositoryError>;
            async fn save(&self, post: &Post) -> Result<(), BaseRepositoryError>;
            async fn delete(&self, id: PostId) -> Result<(), BaseRepositoryError>;
        }
    }
}
