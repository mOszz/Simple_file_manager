use std::fs;
use std::fs::{File, read_to_string};

/// FileHandler provides methods to create, read, write, copy, rename, and delete files.
pub struct FileHandler;

impl FileHandler {
    /// Creates a new file with the specified file name.
    /// # Arguments
    /// * `file_name` - A string slice that holds the name of the file to be created.
    pub fn create(&self, file_name: &str) -> std::io::Result<()> {
        File::create(file_name)?;
        Ok(())
    }

    /// Reads the contents of a file specified by its path and prints the contents to the standard output.
    /// # Arguments
    /// * `file_path` - A string slice that holds the path of the file to be read.
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

    /// Writes the specified content to a file with the given file name.
    /// # Arguments
    /// * `file_name` - A string slice that holds the name of the file to be created or written to.
    /// * `file_content` - A string slice that holds the content to be written to the file.
    pub fn write(&self, file_name: &str, file_content: &str) -> std::io::Result<()> {
        match fs::write(file_name, file_content) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to write file with filename '{}':{}", file_name, err);
                Err(err)
            }
        }
    }

    /// Copies a file to the specified destination.
    /// # Arguments
    /// * `file` - A string slice that holds the path of the file to be copied.
    /// * `destination` - A string slice that holds the destination path where the file will be copied.
    pub fn copy(&self, file: &str, destination: &str) -> std::io::Result<()> {
        match fs::copy(file, destination) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to copy file '{}' to '{}':{}", file, destination, err);
                Err(err)
            }
        }
    }

    /// Renames or moves a file to a new file name or path.
    /// # Arguments
    /// * `file_name` - A string slice that holds the current name or path of the file.
    /// * `new_file_name` - A string slice that holds the new name or path for the file.
    pub fn rename_or_move(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()> {
        match fs::rename(file_name, new_file_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to rename or move file '{}':{}", file_name, err);
                Err(err)
            }
        }
    }

    /// Deletes a file with the specified file name.
    /// # Arguments
    /// * `file_name` - A string slice that holds the name of the file to be deleted.
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