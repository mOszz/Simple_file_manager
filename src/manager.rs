use std::fs;
use std::fs::{create_dir, File, read_dir, read_to_string};
use std::io::Write;

pub trait Operations {
    fn create_file(&self, file_name: &str) -> std::io::Result<()>;
    fn create_directory(&self, dir_name: &str) -> std::io::Result<()>;
    fn list_files(&self, read_dir: &str) -> std::io::Result<()>;
    fn read_file(&self, file_path: &str) -> std::io::Result<()>;
    fn write_file(&self, file_name: &str, file_content: &str) -> std::io::Result<()>;
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

    fn list_files(&self, dir: &str) -> std::io::Result<()> {
        for file in read_dir(dir).unwrap() {
            println!("{}", file.unwrap().path().display());
        }
        Ok(())
    }

    fn read_file(&self, file_path: &str) -> std::io::Result<()> {
        let contents = read_to_string(file_path)
            .expect("Didn't able to read the file");
        println!("{}", contents);
        Ok(())
    }

    fn write_file(&self, file_name: &str, file_content: &str) -> std::io::Result<()> {
        fs::write(file_name, file_content)?;
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

    #[test]
    fn test_list_files() {
        let file_manager = FileManager;
        assert!(file_manager.list_files("./").is_ok());
    }

    #[test]
    fn test_read_file() {
        let file_manager = FileManager;
        assert!(file_manager.read_file("./test_file.txt").is_ok());
    }

    #[test]
    fn test_write_file() {
        let file_manager = FileManager;
        assert!(file_manager.write_file("test_file.txt", "test content").is_ok());
    }
}
