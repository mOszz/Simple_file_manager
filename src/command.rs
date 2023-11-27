use std::{io, process};
use crate::manager::{FileManager, Operations};

pub enum Command {
    List(String),               // ("<directory>")
    CreateFile(String),         // ("<file name>")
    CreateDirectory(String),    // ("<directory name>")
    ReadFile(String),           // ("<file name>")
    DeleteFile(String),         // ("<file name>")
    DeleteDirectory(String),    // ("<directory name>")
    ChangeDirectory(String),    // ("<directory name>")
    WriteFile(String, String),  // ("<file name>", "<content>")
    CopyFile(String, String),   // ("<file name>", "<destination>")
    RenameOrMoveFile(String, String), // ("<file name>", "<new file name>")
    Exit,
    Help,
}

impl Command {
    /// Reads a line of user input from the standard input.
    ///
    /// # Returns
    /// A `String` containing the user input after trimming leading and trailing whitespaces.
    ///
    pub fn read_user_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read in command");

        input.trim().to_string()
    }

    /// Parses a string into a vector of strings by splitting on whitespaces.
    ///
    /// # Arguments
    /// * `input` - A string to be parsed.
    ///
    /// # Returns
    /// A `Vec<String>` containing the parsed words.
    ///
    pub fn parse_user_input(input: &str) -> Vec<String> {
        input.trim().split_whitespace().map(String::from).collect()
    }

    /// Identifies the command and its arguments from a vector of words.
    ///
    /// # Arguments
    /// * `words` - A vector of strings representing words.
    ///
    /// # Returns
    /// A tuple containing the command and its arguments as a `String` and a `Vec<String>` respectively.
    ///
    pub fn identify_args(words: Vec<String>) -> (String, Vec<String>) {
        if words.is_empty() {
            return (String::new(), Vec::new());
        }
        let command = words[0].clone();
        let args = words[1..].to_vec();
        (command, args)
    }

    pub fn execute_command(command: Command, fm: &FileManager) -> io::Result<()> {
        match command {
            Command::List(directory) => FileManager::list_files(fm, &directory),
            Command::CreateFile(file_name) => FileManager::create_file(fm, &file_name),
            Command::CreateDirectory(directory_name) => FileManager::create_directory(fm, &directory_name),
            Command::ReadFile(file_name) => FileManager::read_file(fm, &file_name),
            Command::DeleteFile(file_name) => FileManager::delete_file(fm, &file_name),
            Command::DeleteDirectory(directory_name) => FileManager::delete_directory(fm, &directory_name),
            Command::ChangeDirectory(directory_name) => FileManager::change_directory(fm, &directory_name),
            Command::WriteFile(file_name, content) => FileManager::write_file(fm, &file_name, &content),
            Command::CopyFile(file_name, destination) => FileManager::copy_file(fm, &file_name, &destination),
            Command::RenameOrMoveFile(old_name, new_name) => FileManager::rename_or_move_file(fm, &old_name, &new_name),
            Command::Exit => process::exit(0),
            Command::Help => {
                println!("Help");
                Ok(())
            }
        }
    }
}
