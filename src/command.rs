use std::{io, process};
use crate::manager::{FileManager, Operations};

type CommandAction = Box<dyn Fn(&FileManager, Vec<String>) -> io::Result<()>>;
pub struct Command {
    pub name: String,
    pub command: String,
    pub args: (Vec<String>, bool),
    pub description: String,
    pub action: CommandAction,
}

impl Command {
    pub fn new(name: String, command: String, args: (Vec<String>, bool), description: String, action: CommandAction) -> Self {
        Command {
            name,
            command,
            args,
            description,
            action,
        }
    }

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
        input.split_whitespace().map(String::from).collect()
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

    
    pub fn execute_command(args: Vec<String>, command: Command) {
        let fm = FileManager::new();
        let (expected_args, need_args) = &command.args;
        if *need_args && args.len() != expected_args.len() {
            eprintln!("Usage: {} {}", command.command, expected_args.join(" "));
            return;
        }
        Self::command_action(args, &fm, &command);
    }
    /// Executes a command with the given arguments, FileManager instance, and Command.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of strings representing the command arguments.
    /// * `fm` - A reference to a FileManager instance.
    /// * `cmd` - A reference to a Command instance.
    ///
    fn command_action(mut args: Vec<String>, fm: &FileManager, cmd: &Command) {
        if args.is_empty() {
            args.push("./".to_string());
        }
        match (cmd.action)(fm, args) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Command failed: {}", err);
            }
        }
    }
    pub fn load_command(cmd: &str) -> Option<Command> {
        match cmd {
            "ls" => Some(Command {
                name: "List".to_string(),
                command: "ls".to_string(),
                args: (vec![String::from("<directory>")], false),
                description: "Command to list all directory or specific one".to_string(),
                action: Box::new(|fm, args| FileManager::list_files(fm, &args[0])),
            }),
            "file" => Some(Command {
                name: "Create File".to_string(),
                command: "file".to_string(),
                args: (vec![String::from("<file name>")], true),
                description: "Command to create a file".to_string(),
                action: Box::new(|fm, args| FileManager::create_file(fm, &args[0])),
            }),
            "mkdir" => Some(Command {
                name: "Create Directory".to_string(),
                command: "mkdir".to_string(),
                args: (vec![String::from("<directory name>")], true),
                description: "Command to create a directory".to_string(),
                action: Box::new(|fm, args| FileManager::create_directory(fm, &args[0])),
            }),
            "read" => Some(Command {
                name: "Read File".to_string(),
                command: "read".to_string(),
                args: (vec![String::from("<file name>")], true),
                description: "Command to read a file".to_string(),
                action: Box::new(|fm, args| FileManager::read_file(fm, &args[0])),
            }),
            "rm" => Some(Command {
                name: "Delete File".to_string(),
                command: "rm".to_string(),
                args: (vec![String::from("<file name>")], true),
                description: "Command to delete a file".to_string(),
                action: Box::new(|fm, args| FileManager::delete_file(fm, &args[0])),
            }),
            "rmdir" => Some(Command {
                name: "Delete Directory".to_string(),
                command: "rmdir".to_string(),
                args: (vec![String::from("<directory name>")], true),
                description: "Command to delete a directory".to_string(),
                action: Box::new(|fm, args| FileManager::delete_directory(fm, &args[0])),
            }),
            "cd" => Some(Command {
                name: "Change Directory".to_string(),
                command: "cd".to_string(),
                args: (vec![String::from("<directory name>")], true),
                description: "Command to change the current directory".to_string(),
                action: Box::new(|fm, args| FileManager::change_directory(fm, &args[0])),
            }),
            "write" => Some(Command {
                name: "Write file".to_string(),
                command: "write".to_string(),
                args: (vec![String::from("<file name>"), String::from("<content>")], true),
                description: "Command to write in a file".to_string(),
                action: Box::new(|fm, args| FileManager::write_file(fm, &args[0], &args[1])),
            }),
            "cp" => Some(Command {
                name: "Copy file".to_string(),
                command: "cp".to_string(),
                args: (vec![String::from("<file name>"), String::from("<destination>")], true),
                description: "Command to copy a file".to_string(),
                action: Box::new(|fm, args| FileManager::copy_file(fm, &args[0], &args[1])),
            }),
            "mv" => Some(Command {
                name: "Rename or Move".to_string(),
                command: "mv".to_string(),
                args: (vec![String::from("<file name>"), String::from("<new file name>")], true),
                description: "Command to rename or move a file".to_string(),
                action: Box::new(|fm, args| FileManager::rename_or_move_file(fm, &args[0], &args[1])),
            }),
            "help" => {
                println!("Available commands:\n ls, file, mkdir, read, rm, rmdir, cd, write, cp, mv");
                None
            },
            "exit" => process::exit(0),
            _ => {
                eprintln!("Command not found: {}", cmd);
                None
            } 
        }
    }
}
