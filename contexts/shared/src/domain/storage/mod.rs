use std::fs::File;

#[async_trait::async_trait]
pub trait FileStorageRepository: Send + Sync {
    async fn get(&self, model: String, id: String, filename: String) -> Result<File, String>;
    async fn save(
        &self,
        model: String,
        id: String,
        filename: String,
        file: File,
    ) -> Result<String, String>;
    async fn delete(&self, model: String, id: String, filename: String) -> Result<(), String>;
}

pub mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub FileStorageRepository {}

        #[async_trait::async_trait]
        impl FileStorageRepository for FileStorageRepository {
            async fn get(&self, model: String, id: String, filename: String) -> Result<File, String>;
            async fn save(&self, model: String, id: String, filename: String, file: File) -> Result<String, String>;
            async fn delete(&self, model: String, id: String, filename: String) -> Result<(), String>;
        }
    }
}
