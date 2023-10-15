use std::fs::{create_dir, read_dir, remove_dir_all};

/// DirectoryHandler provides methods to create, list and delete directories.
pub struct DirectoryHandler;

impl DirectoryHandler {
    /// Creates a new directory with the specified directory name.
    /// # Arguments
    /// * `dir_name` - A string slice that holds the name of the directory to be created.
    pub fn create(&self, dir_name: &str) -> std::io::Result<()> {
        match create_dir(dir_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to create directory '{}':{}", dir_name, err);
                Err(err)
            }
        }
    }

    /// Lists the files and directories in the specified directory and prints their paths to the standard output.
    /// # Arguments
    /// * `dir` - A string slice that holds the path of the directory to be listed.
    pub fn list(&self, dir: &str) -> std::io::Result<()> {
        match read_dir(dir) {
            Ok(entries) => {
                for file in entries {
                    println!("{}", file.unwrap().path().display());
                }
            Ok(())
            },
            Err(err) => {
                eprintln!("Failed to list files and directory: {}", err);
                Err(err)
            }
        }
    }

    /// Deletes a directory with the specified directory name.
    /// # Arguments
    /// * `dir_name` - A string slice that holds the name of the directory to be deleted.
    pub fn delete(&self, dir_name: &str) -> std::io::Result<()> {
        match remove_dir_all(dir_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to delete directory '{}':{}", dir_name, err);
                Err(err)
            }
        }
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