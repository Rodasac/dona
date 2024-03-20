use std::{
    fs::{create_dir, File},
    io::Read,
};

use crate::domain::storage::FileStorageRepository;

#[derive(Debug, Clone)]
pub struct DiskFileStorageRepository {
    path: String,
    temp: bool,
}

impl DiskFileStorageRepository {
    pub fn new(path: String, temp: bool) -> Self {
        Self { path, temp }
    }

    pub fn new_temp() -> Self {
        let temp_dir = std::env::temp_dir();
        let id = uuid::Uuid::new_v4();
        let path = format!("{}/storage_{}", temp_dir.display(), id.to_string());
        create_dir(path.clone()).unwrap();

        Self::new(path, true)
    }

    pub fn get_file_path(&self, model: String, id: String, filename: String) -> String {
        format!("{}/{}/{}/{}", self.path, model, id, filename)
    }
}

impl Default for DiskFileStorageRepository {
    fn default() -> Self {
        let pwd = std::env::current_dir().unwrap();
        let path = format!("{}/storage_files", pwd.display());

        Self::new(path, false)
    }
}

impl Drop for DiskFileStorageRepository {
    fn drop(&mut self) {
        if self.temp {
            std::fs::remove_dir_all(&self.path).unwrap_or_default();
        }
    }
}

#[async_trait::async_trait]
impl FileStorageRepository for DiskFileStorageRepository {
    async fn get(&self, model: String, id: String, filename: String) -> Result<File, String> {
        let path = self.get_file_path(model, id, filename);
        if std::fs::metadata(&path).is_err() {
            return Err("File not found".to_string());
        }

        File::open(path).map_err(|e| e.to_string())
    }

    async fn save(
        &self,
        model: String,
        id: String,
        filename: String,
        file: File,
    ) -> Result<String, String> {
        let path = self.get_file_path(model.clone(), id.clone(), filename.clone());

        let mut file = file;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        std::fs::create_dir_all(format!("{}/{}/{}", self.path, model, id))
            .map_err(|e| e.to_string())?;
        std::fs::write(path, buffer).map_err(|e| e.to_string())?;

        Ok(filename.clone())
    }

    async fn delete(&self, model: String, id: String, filename: String) -> Result<(), String> {
        let path = self.get_file_path(model, id, filename);
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, Write};

    #[tokio::test]
    async fn should_get_file() {
        let storage = DiskFileStorageRepository::new_temp();
        let model = "test".to_string();
        let id = "1".to_string();
        let filename = "test.txt".to_string();
        let content = "test".to_string();

        let file_path = storage.get_file_path(model.clone(), id.clone(), filename.clone());
        std::fs::create_dir_all(format!("{}/{}/{}", storage.path, model, id)).unwrap();
        let file = File::create(file_path).unwrap();
        let mut file = file;
        file.write_all(content.as_bytes()).unwrap();

        let mut file = storage
            .get(model.clone(), id.clone(), filename.clone())
            .await
            .unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        assert_eq!(content, String::from_utf8(buffer).unwrap());

        storage
            .delete(model.clone(), id.clone(), filename.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn should_save_file() {
        let storage = DiskFileStorageRepository::new_temp();
        let model = "test".to_string();
        let id = "1".to_string();
        let filename = "test.txt".to_string();
        let content = "test".to_string();

        let mut file_temp = tempfile::tempfile().unwrap();
        file_temp.write_all(content.as_bytes()).unwrap();
        file_temp.seek(std::io::SeekFrom::Start(0)).unwrap();

        let file = storage
            .save(model.clone(), id.clone(), filename.clone(), file_temp)
            .await
            .unwrap();
        assert_eq!(filename, file);

        let mut file = storage
            .get(model.clone(), id.clone(), filename.clone())
            .await
            .unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        assert_eq!(content, String::from_utf8(buffer).unwrap());

        storage
            .delete(model.clone(), id.clone(), filename.clone())
            .await
            .unwrap();
    }
}
