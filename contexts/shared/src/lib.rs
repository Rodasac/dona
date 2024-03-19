pub mod domain;
pub mod infrastructure;

pub const USER_STORAGE_MODEL: &str = "user";

pub const FILE_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "gif"];

pub fn check_file_extension(filename: &str) -> Result<(), String> {
    let extension = filename.split('.').last().unwrap_or_default();

    if FILE_EXTENSIONS.contains(&extension) {
        return Ok(());
    }

    Err("Invalid file extension".to_string())
}
