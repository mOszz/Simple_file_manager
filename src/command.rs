use std::error::Error;
use std::io;
use crate::manager::{FileManager, Operations};

pub struct Command {
    pub name: String,
    pub command: String,
    pub args: (Vec<String>, bool),
    pub description: String,
    pub action: fn(Vec<String>) -> Result<(), Box<dyn Error>>,
}

impl Command {
    pub fn new(name: String, command: String, args: (Vec<String>, bool), description: String, action: fn(Vec<String>) -> Result<(), Box<dyn Error>>) -> Self {
        Command {
            name,
            command,
            args,
            description,
            action,
        }
    }

    pub fn read_user_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read in command");

        input.trim().to_string()
    }

    pub fn parse_user_input(input: &str) -> Vec<String> {
        input.trim().split_whitespace().map(String::from).collect()
    }

    pub fn identify_args(words: Vec<String>) -> (String, Vec<String>) {
        if words.is_empty() {
            return (String::new(), Vec::new());
        }
        let command = words[0].clone();
        let args = words[1..].to_vec();
        (command, args)
    }

    pub fn execute_command(command: &str, args: Vec<String>, commands: &Vec<Command>) {
        if let Some(cmd) = commands.iter().find(|cmd| cmd.command == command) {
            let (expected_args, need_args) = &cmd.args;
            if !need_args {
                if args.len() != expected_args.len() {
                    eprintln!("Usage: {} {}", cmd.command, expected_args.join(" "));
                    return;
                }
            }

            match (cmd.action)(args) {
                Ok(_) => {
                    println!("Command executed successfully");
                }
                Err(err) => {
                    eprintln!("Command failed: {}", err);
                }
            }
        } else {
            println!("Command not found");
        }
    }

    pub fn list_files_action(args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let file_manager = FileManager::new();

        let main_args = if args.len() > 0 {
            args[0].clone()
        } else {
            "./".to_string()
        };

        match file_manager.list_files(&main_args) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
