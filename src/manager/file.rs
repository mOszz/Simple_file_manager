use std::fs;
use std::fs::{File, read_to_string};

pub struct FileHandler;

impl FileHandler {
    pub fn create(&self, file_name: &str) -> std::io::Result<()> {
        File::create(file_name)?;
        Ok(())
    }

    pub fn read(&self, file_path: &str) -> std::io::Result<()> {
        match read_to_string(file_path) {
            Ok(contents) => {
                println!("{}", contents);
                Ok(())
            },
            Err(err) => {
                eprintln!("Failed to read the file: {}", err);
                Err(err)
            }
        }
    }

    pub fn write(&self, file_name: &str, file_content: &str) -> std::io::Result<()> {
        match fs::write(file_name, file_content) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to write file with filename '{}':{}", file_name, err);
                Err(err)
            }
        }
    }

    pub fn copy(&self, file: &str, destination: &str) -> std::io::Result<()> {
        match fs::copy(file, destination) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to copy file '{}' to '{}':{}", file, destination, err);
                Err(err)
            }
        }
    }

    pub fn rename_or_move(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()> {
        match fs::rename(file_name, new_file_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to rename or move file '{}':{}", file_name, err);
                Err(err)
            }
        }
    }

    pub fn delete(&self, file_name: &str) -> std::io::Result<()> {
        match fs::remove_file(file_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to remove file '{}':{}", file_name, err);
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_file() {
        let file_handler = FileHandler;
        assert!(file_handler.create("test_file.txt").is_ok());
    }
    #[test]
    fn test_read_file() {
        let file_handler = FileHandler;
        assert!(file_handler.read("./src/main.rs").is_ok());
    }
    #[test]
    fn test_write_file() {
        let file_handler = FileHandler;
        assert!(file_handler.write("test_file.txt", "test content").is_ok());
    }
    #[test]
    fn test_copy_file() {
        let file_handler = FileHandler;
        assert!(file_handler.copy("test_file.txt", "src/test.txt").is_ok());
    }
    #[test]
    fn test_rename_or_move_file() {
        let file_handler = FileHandler;
        assert!(file_handler.rename_or_move("src/test.txt", "src/new_test_file.txt").is_ok());
    }
    #[test]
    fn test_delete_file() {
        let file_handler = FileHandler;
        assert!(file_handler.delete("test_file.txt").is_ok());
    }
}