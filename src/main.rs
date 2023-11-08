pub mod manager;
pub mod command;

use std::error::Error;
use std::io;
use std::io::{stdout, Write};
use manager::{Operations, FileManager};
use command::Command;

fn app_running() {
    let prompt_char = '🦀';

    print!("{} ", prompt_char);
    stdout().flush().unwrap();
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read in command");

    return input.trim().to_string();
}

fn parse_user_input(input: &str) -> Vec<String> {
    input.trim().split_whitespace().map(String::from).collect()
}

fn identify_args(words: Vec<String>) -> (String, Vec<String>) {
    if words.is_empty() {
        return (String::new(), Vec::new());
    }
    let command = words[0].clone();
    let args = words[1..].to_vec();
    (command, args)
}

fn execute_command(command: &str, args: Vec<String>, commands: &Vec<Command>) {
    if let Some(cmd) = commands.iter().find(|cmd| cmd.command == command) {
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

fn list_files_action(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file_manager = FileManager::new();
    match file_manager.list_files("/.") {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

fn main() {
    let commands: Vec<Command> = vec![
        Command {
            name: "List".to_string(),
            command: "l".to_string(),
            args: vec![String::from("./")],
            description: "Command to list all directory or specific one".to_string(),
            action: list_files_action,
        },


    ];

    loop {
        app_running();

        let user_input = read_user_input();
        let parsed_input = parse_user_input(&user_input);
        let (command, args) = identify_args(parsed_input);

        execute_command(&command, args, &commands);
    }

    //file_manager.list_files("src").expect("TODO: panic message");
    //file_manager.create_file("test.txt");
    //file_manager.create_file("C:/Users/victo/Documents/test.txt");
    //file_manager.read_file("C:/Users/victo/Documents/dev/rust/simple_file_manager/src/main.rs");
    //file_manager.write_file("C:/Users/victo/Documents/dev/rust/simple_file_manager/test.txt", "udidhbdi");
    //file_manager.copy_file("test.txt", "src/test.txt");
    //file_manager.rename_or_move_file("dddd", "dddd");
    //file_manager.delete_file("sdf");
    //file_manager.create_directory("./src");
    //file_manager.list_files("/toto/failed test");
    //file_manager.delete_file("./test/delete/folder");
}
