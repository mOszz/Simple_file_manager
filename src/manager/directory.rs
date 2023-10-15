use std::fs;
use std::fs::{create_dir, read_dir};

pub struct DirectoryHandler;

impl DirectoryHandler {
    pub fn create(&self, dir_name: &str) -> std::io::Result<()> {
        create_dir(dir_name)?;
        Ok(())
    }

    pub fn list(&self, dir: &str) -> std::io::Result<()> {
        for file in read_dir(dir).unwrap() {
            println!("{}", file.unwrap().path().display());
        }
        Ok(())
    }

    pub fn delete(&self, dir_name: &str) -> std::io::Result<()> {
        fs::remove_dir_all(dir_name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_directory() {
        let directory_handler = DirectoryHandler;
        assert!(directory_handler.create("dir_name").is_ok());
    }

    #[test]
    fn test_list_files() {
        let directory_handler = DirectoryHandler;
        assert!(directory_handler.list("./").is_ok());
    }
    #[test]
    fn test_delete_directory() {
        let directory_handler = DirectoryHandler;
        assert!(directory_handler.delete("./dir_name").is_ok());
    }
}


