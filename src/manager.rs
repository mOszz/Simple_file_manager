mod file;
mod directory;

use std::env;
use file::FileHandler;
use directory::DirectoryHandler;

pub trait Operations {
    fn new() -> Self;
    fn create_file(&self, file_name: &str) -> std::io::Result<()>;
    fn create_directory(&self, dir_name: &str) -> std::io::Result<()>;
    fn list_files(&self, read_dir: &str) -> std::io::Result<()>;
    fn read_file(&self, file_path: &str) -> std::io::Result<()>;
    fn write_file(&self, file_name: &str, file_content: &str) -> std::io::Result<()>;
    fn copy_file(&self, file: &str, destination: &str) -> std::io::Result<()>;
    fn rename_or_move_file(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()>;
    fn delete_file(&self, file_name: &str) -> std::io::Result<()>;
    fn delete_directory(&self, dir_name: &str) -> std::io::Result<()>;
    fn change_directory(&self, dir_name: &str) -> std::io::Result<()>;
}

pub struct FileManager {
    file_handler: FileHandler,
    directory_handler: DirectoryHandler,
}

impl Operations for FileManager {

    fn new() -> Self {
        FileManager {
            file_handler: FileHandler,
            directory_handler: DirectoryHandler,
        }
    }

    fn create_file(&self, file_name: &str) -> std::io::Result<()> {
        self.file_handler.create(file_name)
    }

    fn create_directory(&self, dir_name: &str) -> std::io::Result<()> {
        self.directory_handler.create(dir_name)
    }

    fn list_files(&self, dir: &str) -> std::io::Result<()> {
        self.directory_handler.list(dir)
    }

    fn read_file(&self, file_path: &str) -> std::io::Result<()> {
        self.file_handler.read(file_path)
    }

    fn write_file(&self, file_name: &str, file_content: &str) -> std::io::Result<()> {
        self.file_handler.write(file_name, file_content)
    }

    fn copy_file(&self, file: &str, destination: &str) -> std::io::Result<()> {
        self.file_handler.copy(file, destination)
    }

    fn rename_or_move_file(&self, file_name: &str, new_file_name: &str) -> std::io::Result<()> {
        self.file_handler.rename_or_move(file_name, new_file_name)
    }

    fn delete_file(&self, file_name: &str) -> std::io::Result<()> {
        self.file_handler.delete(file_name)
    }

    fn delete_directory(&self, dir_name: &str) -> std::io::Result<()> {
        self.directory_handler.delete(dir_name)
    }

    fn change_directory(&self, dir_name: &str) -> std::io::Result<()> {
        match env::set_current_dir(dir_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to change current directory '{}':{}", dir_name, err);
                Err(err)
            }
        }
    }
}
