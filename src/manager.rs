use std::fs;
use std::fs::{create_dir, File, read_dir, read_to_string};

pub trait Operations {
    fn create_file(&self, file_name: &str) -> std::io::Result<()>;
    fn create_directory(&self, dir_name: &str) -> std::io::Result<()>;
    fn list_files(&self, read_dir: &str) -> std::io::Result<()>;
    fn read_file(&self, file_path: &str) -> std::io::Result<()>;
    fn write_file(&self, file_name: &str, file_content: &str) -> std::io::Result<()>;
    fn copy_file(&self, file: &str, destination: &str) -> std::io::Result<()>;
    fn rename_or_move_file(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()>;
    fn delete_file(&self, file_name: &str) -> std::io::Result<()>;
    fn delete_directory(&self, dir_name: &str) -> std::io::Result<()>;
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

    fn copy_file(&self, file: &str, destination: &str) -> std::io::Result<()> {
        fs::copy(file, destination)?;
        Ok(())
    }

    fn rename_or_move_file(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()> {
        fs::rename(file_name, new_file_name)?;
        Ok(())
    }

    fn delete_file(&self, file_name: &str) -> std::io::Result<()> {
        fs::remove_file(file_name)?;
        Ok(())
    }

    fn delete_directory(&self, dir_name: &str) -> std::io::Result<()> {
        fs::remove_dir_all(dir_name)?;
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

    #[test]
    fn test_copy_file() {
        let file_manager = FileManager;
        assert!(file_manager.copy_file("test_file.txt", "src/test.txt").is_ok());
    }
    #[test]
    fn test_rename_or_move_file() {
        let file_manager = FileManager;
        assert!(file_manager.rename_or_move_file("new_test_file.txt", "src/new_test_file.txt").is_ok());
    }
    #[test]
    fn test_delete_file() {
        let file_manager = FileManager;
        assert!(file_manager.delete_file("test_file.txt").is_ok());
    }

    #[test]
    fn test_delete_directory() {
        let file_manager = FileManager;
        assert!(file_manager.delete_directory("./dir_name").is_ok());
    }
}
