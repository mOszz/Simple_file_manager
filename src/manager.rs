use std::fs::{create_dir, File};

pub trait Operations {
    fn create_file(&self, file_name: &str) -> std::io::Result<()>;
    fn create_directory(&self, dir_name: &str) -> std::io::Result<()>;

}

pub struct FileManager;

impl Operations for FileManager {

    fn create_file(&self, file_name: &str) -> std::io::Result<()> {
        File::create(file_name)?;
        Ok(())
    }

    fn create_directory(&self, dir_name: &str) -> std::io::Result<()> {
        create_dir(dir_name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let file_manager = FileManager;
        assert!(file_manager.create_file("test_file.txt").is_ok());
    }

    #[test]
    fn test_create_directory() {
        let file_manager = FileManager;
        assert!(file_manager.create_directory("dir_name").is_ok());
    }
}
