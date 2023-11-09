pub mod manager;
pub mod command;

use std::error::Error;
use std::io::{stdout, Write};
use manager::{Operations, FileManager};
use command::Command;

fn app_running() {
    let prompt_char = '🦀';
    print!("{} ", prompt_char);
    stdout().flush().unwrap();
}

fn main() {
    let commands: Vec<Command> = vec![
        Command {
            name: "List".to_string(),
            command: "l".to_string(),
            args: (vec![String::from("<directory>")], true),
            description: "Command to list all directory or specific one".to_string(),
            action: Command::list_files_action,
        },
    ];

    loop {
        app_running();

        let user_input = Command::read_user_input();
        let parsed_input = Command::parse_user_input(&user_input);
        let (command, args) = Command::identify_args(parsed_input);

        Command::execute_command(&command, args, &commands);
    }
}
    //file_manager.list_files("src").expect("TODO: panic message");
    //file_manager.create_file("test.txt");
    //file_manager.create_file("C:/Users/victor/Documents/test.txt");
    //file_manager.read_file("C:/Users/victor/Documents/dev/rust/simple_file_manager/src/main.rs");
    //file_manager.write_file("C:/Users/victor/Documents/dev/rust/simple_file_manager/test.txt", "udidhbdi");
    //file_manager.copy_file("test.txt", "src/test.txt");
    //file_manager.rename_or_move_file("dddd", "dddd");
    //file_manager.delete_file("sdf");
    //file_manager.create_directory("./src");
    //file_manager.list_files("/toto/failed test");
    //file_manager.delete_file("./test/delete/folder");

