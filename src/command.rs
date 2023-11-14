use std::{io, process};
use crate::manager::{FileManager, Operations};

//type CommandAction = fn(&FileManager, &str) -> io::Result<()>;
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

    /// Executes a command with the given arguments and Command instances.
    ///
    /// # Arguments
    ///
    /// * `command` - A string representing the command to be executed.
    /// * `args` - A vector of strings representing the command arguments.
    /// * `commands` - A reference to a vector of Command instances.
    ///
    pub fn execute_command(command: &str, args: Vec<String>, commands: &Vec<Command>) {
        let fm = FileManager::new();
        if let Some(cmd) = commands.iter().find(|cmd| cmd.command == command) {
            let (expected_args, need_args) = &cmd.args;
            if *need_args {
                if args.len() != expected_args.len() {
                    eprintln!("Usage: {} {}", cmd.command, expected_args.join(" "));
                    return;
                }
            }
            Self::command_action(args, &fm, cmd);
        } else if command == "exit" {
            process::exit(0);
        } else if command == "help" {
            for cmd in commands {
                print!("{}, ", cmd.command);
            }
            print!("\n")
        }
        else {
            println!("Command not found");
        }
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
        if args.len() < 1 {
            args.push("./".to_string());
        }
        match (cmd.action)(&fm, args) {
            Ok(_) => {
                //println!("Command executed successfully");
            }
            Err(err) => {
                eprintln!("Command failed: {}", err);
            }
        }
    }
}
